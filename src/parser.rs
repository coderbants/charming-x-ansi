//! Cleanroom Rust port of upstream Go source file: `ansi/parser.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! Also covers (same upstream module):
//! `ansi/parser_handler.go`, `ansi/parser_decode.go`, `ansi/parser_sync.go`,
//! `ansi/c0.go`, `ansi/c1.go`, `ansi/ansi.go`, `ansi/ascii.go`,
//! `ansi/parser/const.go`, `ansi/parser/seq.go`, `ansi/parser/transition_table.go`
//!
//! <public-docs>
//! A DEC ANSI compatible escape-sequence state machine: the `Parser` with its
//! `Handler` callbacks, the packed `Cmd`/`Param`/`Params` types, and the
//! `DecodeSequence` incremental sequence decoder used by ultraviolet's event
//! decoder.
//! </public-docs>

use std::io::{self, Write};
use std::sync::OnceLock;

/// Action is a DEC ANSI parser action.
pub type Action = u8;

/// These are the actions that the parser can take.
pub const NONE_ACTION: Action = 0;
/// ClearAction clears the parser state.
pub const CLEAR_ACTION: Action = 1;
/// CollectAction collects a byte.
pub const COLLECT_ACTION: Action = 2;
/// PrefixAction collects a private prefix.
pub const PREFIX_ACTION: Action = 3;
/// DispatchAction dispatches a sequence.
pub const DISPATCH_ACTION: Action = 4;
/// ExecuteAction executes a control character.
pub const EXECUTE_ACTION: Action = 5;
/// StartAction starts a data string.
pub const START_ACTION: Action = 6;
/// PutAction puts a byte into the data string.
pub const PUT_ACTION: Action = 7;
/// ParamAction collects a parameter.
pub const PARAM_ACTION: Action = 8;
/// PrintAction prints a rune.
pub const PRINT_ACTION: Action = 9;

/// IgnoreAction is an alias for [NONE_ACTION].
pub const IGNORE_ACTION: Action = NONE_ACTION;

/// State is a DEC ANSI parser state.
pub type State = u8;

/// These are the states that the parser can be in.
pub const GROUND_STATE: State = 0;
/// CsiEntryState is the CSI entry state.
pub const CSI_ENTRY_STATE: State = 1;
/// CsiIntermediateState is the CSI intermediate state.
pub const CSI_INTERMEDIATE_STATE: State = 2;
/// CsiParamState is the CSI parameter state.
pub const CSI_PARAM_STATE: State = 3;
/// DcsEntryState is the DCS entry state.
pub const DCS_ENTRY_STATE: State = 4;
/// DcsIntermediateState is the DCS intermediate state.
pub const DCS_INTERMEDIATE_STATE: State = 5;
/// DcsParamState is the DCS parameter state.
pub const DCS_PARAM_STATE: State = 6;
/// DcsStringState is the DCS string state.
pub const DCS_STRING_STATE: State = 7;
/// EscapeState is the ESC state.
pub const ESCAPE_STATE: State = 8;
/// EscapeIntermediateState is the ESC intermediate state.
pub const ESCAPE_INTERMEDIATE_STATE: State = 9;
/// OscStringState is the OSC string state.
pub const OSC_STRING_STATE: State = 10;
/// SosStringState is the SOS string state.
pub const SOS_STRING_STATE: State = 11;
/// PmStringState is the PM string state.
pub const PM_STRING_STATE: State = 12;
/// ApcStringState is the APC string state.
pub const APC_STRING_STATE: State = 13;
/// Utf8State is not part of the DEC ANSI standard. It is used to handle
/// UTF-8 sequences.
pub const UTF8_STATE: State = 14;

/// StateNames provides string names for parser states.
pub fn state_name(state: State) -> &'static str {
    const NAMES: [&str; 15] = [
        "GroundState",
        "CsiEntryState",
        "CsiIntermediateState",
        "CsiParamState",
        "DcsEntryState",
        "DcsIntermediateState",
        "DcsParamState",
        "DcsStringState",
        "EscapeState",
        "EscapeIntermediateState",
        "OscStringState",
        "SosStringState",
        "PmStringState",
        "ApcStringState",
        "Utf8State",
    ];
    NAMES.get(state as usize).copied().unwrap_or("UnknownState")
}

/// Shift and masks for sequence parameters and intermediates.
pub const PREFIX_SHIFT: i32 = 8;
/// Shift and masks for sequence parameters and intermediates.
pub const INTERMED_SHIFT: i32 = 16;
/// FinalMask masks the final byte of a command.
pub const FINAL_MASK: i32 = 0xff;
/// HasMoreFlag marks a parameter that has following sub-parameters.
pub const HAS_MORE_FLAG: i32 = i32::MIN;
/// ParamMask masks the parameter value, clearing [HAS_MORE_FLAG].
pub const PARAM_MASK: i32 = !HAS_MORE_FLAG;
/// MissingParam is the value of a missing parameter.
pub const MISSING_PARAM: i32 = PARAM_MASK;
/// MissingCommand is the value of a missing command.
pub const MISSING_COMMAND: i32 = MISSING_PARAM;
/// MaxParam is the maximum value a parameter can have.
pub const MAX_PARAM: i32 = u16::MAX as i32;

/// MaxParamsSize is the maximum number of parameters a sequence can have.
pub const MAX_PARAMS_SIZE: usize = 32;
/// DefaultParamValue is the default value used for missing parameters.
pub const DEFAULT_PARAM_VALUE: i32 = 0;

/// TransitionActionShift shifts the action in a transition table value.
const TRANSITION_ACTION_SHIFT: u8 = 4;
/// TransitionStateMask masks the state in a transition table value.
const TRANSITION_STATE_MASK: u8 = 15;
/// IndexStateShift shifts the state in a transition table index.
const INDEX_STATE_SHIFT: usize = 8;

/// DefaultTableSize is the default size of the transition table.
const DEFAULT_TABLE_SIZE: usize = 4096;

/// Returns the next state and action for the given state and byte.
fn transition(state: State, code: u8) -> (State, Action) {
    let table = transition_table();
    let index = (state as usize) << INDEX_STATE_SHIFT | code as usize;
    let value = table[index];
    (
        value & TRANSITION_STATE_MASK,
        value >> TRANSITION_ACTION_SHIFT,
    )
}

fn transition_table() -> &'static [u8; DEFAULT_TABLE_SIZE] {
    static TABLE: OnceLock<Box<[u8; DEFAULT_TABLE_SIZE]>> = OnceLock::new();
    TABLE.get_or_init(generate_transition_table)
}

/// Generates a DEC ANSI transition table compatible with the VT500-series of
/// terminals.
fn generate_transition_table() -> Box<[u8; DEFAULT_TABLE_SIZE]> {
    let mut table = Box::new([0u8; DEFAULT_TABLE_SIZE]);

    // SetDefault(NoneAction, GroundState): zeros are already default.

    // Anywhere
    for state in GROUND_STATE..=UTF8_STATE {
        // Anywhere -> Ground
        add_many(
            &mut table,
            &[0x18, 0x1a, 0x99, 0x9a],
            state,
            EXECUTE_ACTION,
            GROUND_STATE,
        );
        add_range(&mut table, 0x80, 0x8F, state, EXECUTE_ACTION, GROUND_STATE);
        add_range(&mut table, 0x90, 0x97, state, EXECUTE_ACTION, GROUND_STATE);
        add_one(&mut table, 0x9C, state, EXECUTE_ACTION, GROUND_STATE);
        // Anywhere -> Escape
        add_one(&mut table, 0x1B, state, CLEAR_ACTION, ESCAPE_STATE);
        // Anywhere -> SosStringState
        add_one(&mut table, 0x98, state, START_ACTION, SOS_STRING_STATE);
        // Anywhere -> PmStringState
        add_one(&mut table, 0x9E, state, START_ACTION, PM_STRING_STATE);
        // Anywhere -> ApcStringState
        add_one(&mut table, 0x9F, state, START_ACTION, APC_STRING_STATE);
        // Anywhere -> CsiEntry
        add_one(&mut table, 0x9B, state, CLEAR_ACTION, CSI_ENTRY_STATE);
        // Anywhere -> DcsEntry
        add_one(&mut table, 0x90, state, CLEAR_ACTION, DCS_ENTRY_STATE);
        // Anywhere -> OscString
        add_one(&mut table, 0x9D, state, START_ACTION, OSC_STRING_STATE);
        // Anywhere -> Utf8
        add_range(&mut table, 0xC2, 0xDF, state, COLLECT_ACTION, UTF8_STATE);
        add_range(&mut table, 0xE0, 0xEF, state, COLLECT_ACTION, UTF8_STATE);
        add_range(&mut table, 0xF0, 0xF4, state, COLLECT_ACTION, UTF8_STATE);
    }

    // Ground
    add_range(
        &mut table,
        0x00,
        0x17,
        GROUND_STATE,
        EXECUTE_ACTION,
        GROUND_STATE,
    );
    add_one(&mut table, 0x19, GROUND_STATE, EXECUTE_ACTION, GROUND_STATE);
    add_range(
        &mut table,
        0x1C,
        0x1F,
        GROUND_STATE,
        EXECUTE_ACTION,
        GROUND_STATE,
    );
    add_range(
        &mut table,
        0x20,
        0x7E,
        GROUND_STATE,
        PRINT_ACTION,
        GROUND_STATE,
    );
    add_one(&mut table, 0x7F, GROUND_STATE, EXECUTE_ACTION, GROUND_STATE);

    // EscapeIntermediate
    add_range(
        &mut table,
        0x00,
        0x17,
        ESCAPE_INTERMEDIATE_STATE,
        EXECUTE_ACTION,
        ESCAPE_INTERMEDIATE_STATE,
    );
    add_one(
        &mut table,
        0x19,
        ESCAPE_INTERMEDIATE_STATE,
        EXECUTE_ACTION,
        ESCAPE_INTERMEDIATE_STATE,
    );
    add_range(
        &mut table,
        0x1C,
        0x1F,
        ESCAPE_INTERMEDIATE_STATE,
        EXECUTE_ACTION,
        ESCAPE_INTERMEDIATE_STATE,
    );
    add_range(
        &mut table,
        0x20,
        0x2F,
        ESCAPE_INTERMEDIATE_STATE,
        COLLECT_ACTION,
        ESCAPE_INTERMEDIATE_STATE,
    );
    add_one(
        &mut table,
        0x7F,
        ESCAPE_INTERMEDIATE_STATE,
        IGNORE_ACTION,
        ESCAPE_INTERMEDIATE_STATE,
    );
    // EscapeIntermediate -> Ground
    add_range(
        &mut table,
        0x30,
        0x7E,
        ESCAPE_INTERMEDIATE_STATE,
        DISPATCH_ACTION,
        GROUND_STATE,
    );

    // Escape
    add_range(
        &mut table,
        0x00,
        0x17,
        ESCAPE_STATE,
        EXECUTE_ACTION,
        ESCAPE_STATE,
    );
    add_one(&mut table, 0x19, ESCAPE_STATE, EXECUTE_ACTION, ESCAPE_STATE);
    add_range(
        &mut table,
        0x1C,
        0x1F,
        ESCAPE_STATE,
        EXECUTE_ACTION,
        ESCAPE_STATE,
    );
    add_one(&mut table, 0x7F, ESCAPE_STATE, IGNORE_ACTION, ESCAPE_STATE);
    // Escape -> Ground
    add_range(
        &mut table,
        0x30,
        0x4F,
        ESCAPE_STATE,
        DISPATCH_ACTION,
        GROUND_STATE,
    );
    add_range(
        &mut table,
        0x51,
        0x57,
        ESCAPE_STATE,
        DISPATCH_ACTION,
        GROUND_STATE,
    );
    add_one(
        &mut table,
        0x59,
        ESCAPE_STATE,
        DISPATCH_ACTION,
        GROUND_STATE,
    );
    add_one(
        &mut table,
        0x5A,
        ESCAPE_STATE,
        DISPATCH_ACTION,
        GROUND_STATE,
    );
    add_one(
        &mut table,
        0x5C,
        ESCAPE_STATE,
        DISPATCH_ACTION,
        GROUND_STATE,
    );
    add_range(
        &mut table,
        0x60,
        0x7E,
        ESCAPE_STATE,
        DISPATCH_ACTION,
        GROUND_STATE,
    );
    // Escape -> Escape_intermediate
    add_range(
        &mut table,
        0x20,
        0x2F,
        ESCAPE_STATE,
        COLLECT_ACTION,
        ESCAPE_INTERMEDIATE_STATE,
    );
    // Escape -> Sos_pm_apc_string
    add_one(
        &mut table,
        b'X',
        ESCAPE_STATE,
        START_ACTION,
        SOS_STRING_STATE,
    );
    add_one(
        &mut table,
        b'^',
        ESCAPE_STATE,
        START_ACTION,
        PM_STRING_STATE,
    );
    add_one(
        &mut table,
        b'_',
        ESCAPE_STATE,
        START_ACTION,
        APC_STRING_STATE,
    );
    // Escape -> Dcs_entry
    add_one(
        &mut table,
        b'P',
        ESCAPE_STATE,
        CLEAR_ACTION,
        DCS_ENTRY_STATE,
    );
    // Escape -> Csi_entry
    add_one(
        &mut table,
        b'[',
        ESCAPE_STATE,
        CLEAR_ACTION,
        CSI_ENTRY_STATE,
    );
    // Escape -> Osc_string
    add_one(
        &mut table,
        b']',
        ESCAPE_STATE,
        START_ACTION,
        OSC_STRING_STATE,
    );

    // Sos_pm_apc_string
    for state in SOS_STRING_STATE..=APC_STRING_STATE {
        add_range(&mut table, 0x00, 0x17, state, PUT_ACTION, state);
        add_one(&mut table, 0x19, state, PUT_ACTION, state);
        add_range(&mut table, 0x1C, 0x1F, state, PUT_ACTION, state);
        add_range(&mut table, 0x20, 0x7F, state, PUT_ACTION, state);
        // ESC, ST, CAN, and SUB terminate the sequence
        add_one(&mut table, 0x1B, state, DISPATCH_ACTION, ESCAPE_STATE);
        add_one(&mut table, 0x9C, state, DISPATCH_ACTION, GROUND_STATE);
        add_many(
            &mut table,
            &[0x18, 0x1A],
            state,
            IGNORE_ACTION,
            GROUND_STATE,
        );
    }

    // Dcs_entry
    add_range(
        &mut table,
        0x00,
        0x07,
        DCS_ENTRY_STATE,
        IGNORE_ACTION,
        DCS_ENTRY_STATE,
    );
    add_range(
        &mut table,
        0x0E,
        0x17,
        DCS_ENTRY_STATE,
        IGNORE_ACTION,
        DCS_ENTRY_STATE,
    );
    add_one(
        &mut table,
        0x19,
        DCS_ENTRY_STATE,
        IGNORE_ACTION,
        DCS_ENTRY_STATE,
    );
    add_range(
        &mut table,
        0x1C,
        0x1F,
        DCS_ENTRY_STATE,
        IGNORE_ACTION,
        DCS_ENTRY_STATE,
    );
    add_one(
        &mut table,
        0x7F,
        DCS_ENTRY_STATE,
        IGNORE_ACTION,
        DCS_ENTRY_STATE,
    );
    // Dcs_entry -> Dcs_intermediate
    add_range(
        &mut table,
        0x20,
        0x2F,
        DCS_ENTRY_STATE,
        COLLECT_ACTION,
        DCS_INTERMEDIATE_STATE,
    );
    // Dcs_entry -> Dcs_param
    add_range(
        &mut table,
        0x30,
        0x3B,
        DCS_ENTRY_STATE,
        PARAM_ACTION,
        DCS_PARAM_STATE,
    );
    add_range(
        &mut table,
        0x3C,
        0x3F,
        DCS_ENTRY_STATE,
        PREFIX_ACTION,
        DCS_PARAM_STATE,
    );
    // Dcs_entry -> Dcs_passthrough
    add_range(
        &mut table,
        0x08,
        0x0D,
        DCS_ENTRY_STATE,
        PUT_ACTION,
        DCS_STRING_STATE,
    );
    add_one(
        &mut table,
        0x1B,
        DCS_ENTRY_STATE,
        PUT_ACTION,
        DCS_STRING_STATE,
    );
    add_range(
        &mut table,
        0x40,
        0x7E,
        DCS_ENTRY_STATE,
        START_ACTION,
        DCS_STRING_STATE,
    );

    // Dcs_intermediate
    add_range(
        &mut table,
        0x00,
        0x17,
        DCS_INTERMEDIATE_STATE,
        IGNORE_ACTION,
        DCS_INTERMEDIATE_STATE,
    );
    add_one(
        &mut table,
        0x19,
        DCS_INTERMEDIATE_STATE,
        IGNORE_ACTION,
        DCS_INTERMEDIATE_STATE,
    );
    add_range(
        &mut table,
        0x1C,
        0x1F,
        DCS_INTERMEDIATE_STATE,
        IGNORE_ACTION,
        DCS_INTERMEDIATE_STATE,
    );
    add_range(
        &mut table,
        0x20,
        0x2F,
        DCS_INTERMEDIATE_STATE,
        COLLECT_ACTION,
        DCS_INTERMEDIATE_STATE,
    );
    add_one(
        &mut table,
        0x7F,
        DCS_INTERMEDIATE_STATE,
        IGNORE_ACTION,
        DCS_INTERMEDIATE_STATE,
    );
    // Dcs_intermediate -> Dcs_passthrough
    add_range(
        &mut table,
        0x30,
        0x3F,
        DCS_INTERMEDIATE_STATE,
        START_ACTION,
        DCS_STRING_STATE,
    );
    add_range(
        &mut table,
        0x40,
        0x7E,
        DCS_INTERMEDIATE_STATE,
        START_ACTION,
        DCS_STRING_STATE,
    );

    // Dcs_param
    add_range(
        &mut table,
        0x00,
        0x17,
        DCS_PARAM_STATE,
        IGNORE_ACTION,
        DCS_PARAM_STATE,
    );
    add_one(
        &mut table,
        0x19,
        DCS_PARAM_STATE,
        IGNORE_ACTION,
        DCS_PARAM_STATE,
    );
    add_range(
        &mut table,
        0x1C,
        0x1F,
        DCS_PARAM_STATE,
        IGNORE_ACTION,
        DCS_PARAM_STATE,
    );
    add_range(
        &mut table,
        0x30,
        0x3B,
        DCS_PARAM_STATE,
        PARAM_ACTION,
        DCS_PARAM_STATE,
    );
    add_one(
        &mut table,
        0x7F,
        DCS_PARAM_STATE,
        IGNORE_ACTION,
        DCS_PARAM_STATE,
    );
    add_range(
        &mut table,
        0x3C,
        0x3F,
        DCS_PARAM_STATE,
        IGNORE_ACTION,
        DCS_PARAM_STATE,
    );
    // Dcs_param -> Dcs_intermediate
    add_range(
        &mut table,
        0x20,
        0x2F,
        DCS_PARAM_STATE,
        COLLECT_ACTION,
        DCS_INTERMEDIATE_STATE,
    );
    // Dcs_param -> Dcs_passthrough
    add_range(
        &mut table,
        0x40,
        0x7E,
        DCS_PARAM_STATE,
        START_ACTION,
        DCS_STRING_STATE,
    );

    // Dcs_passthrough
    add_range(
        &mut table,
        0x00,
        0x17,
        DCS_STRING_STATE,
        PUT_ACTION,
        DCS_STRING_STATE,
    );
    add_one(
        &mut table,
        0x19,
        DCS_STRING_STATE,
        PUT_ACTION,
        DCS_STRING_STATE,
    );
    add_range(
        &mut table,
        0x1C,
        0x1F,
        DCS_STRING_STATE,
        PUT_ACTION,
        DCS_STRING_STATE,
    );
    add_range(
        &mut table,
        0x20,
        0x7E,
        DCS_STRING_STATE,
        PUT_ACTION,
        DCS_STRING_STATE,
    );
    add_one(
        &mut table,
        0x7F,
        DCS_STRING_STATE,
        PUT_ACTION,
        DCS_STRING_STATE,
    );
    add_range(
        &mut table,
        0x80,
        0xFF,
        DCS_STRING_STATE,
        PUT_ACTION,
        DCS_STRING_STATE,
    );
    // ST, CAN, SUB, and ESC terminate the sequence
    add_one(
        &mut table,
        0x1B,
        DCS_STRING_STATE,
        DISPATCH_ACTION,
        ESCAPE_STATE,
    );
    add_one(
        &mut table,
        0x9C,
        DCS_STRING_STATE,
        DISPATCH_ACTION,
        GROUND_STATE,
    );
    add_many(
        &mut table,
        &[0x18, 0x1A],
        DCS_STRING_STATE,
        IGNORE_ACTION,
        GROUND_STATE,
    );

    // Csi_param
    add_range(
        &mut table,
        0x00,
        0x17,
        CSI_PARAM_STATE,
        EXECUTE_ACTION,
        CSI_PARAM_STATE,
    );
    add_one(
        &mut table,
        0x19,
        CSI_PARAM_STATE,
        EXECUTE_ACTION,
        CSI_PARAM_STATE,
    );
    add_range(
        &mut table,
        0x1C,
        0x1F,
        CSI_PARAM_STATE,
        EXECUTE_ACTION,
        CSI_PARAM_STATE,
    );
    add_range(
        &mut table,
        0x30,
        0x3B,
        CSI_PARAM_STATE,
        PARAM_ACTION,
        CSI_PARAM_STATE,
    );
    add_one(
        &mut table,
        0x7F,
        CSI_PARAM_STATE,
        IGNORE_ACTION,
        CSI_PARAM_STATE,
    );
    add_range(
        &mut table,
        0x3C,
        0x3F,
        CSI_PARAM_STATE,
        IGNORE_ACTION,
        CSI_PARAM_STATE,
    );
    // Csi_param -> Ground
    add_range(
        &mut table,
        0x40,
        0x7E,
        CSI_PARAM_STATE,
        DISPATCH_ACTION,
        GROUND_STATE,
    );
    // Csi_param -> Csi_intermediate
    add_range(
        &mut table,
        0x20,
        0x2F,
        CSI_PARAM_STATE,
        COLLECT_ACTION,
        CSI_INTERMEDIATE_STATE,
    );

    // Csi_intermediate
    add_range(
        &mut table,
        0x00,
        0x17,
        CSI_INTERMEDIATE_STATE,
        EXECUTE_ACTION,
        CSI_INTERMEDIATE_STATE,
    );
    add_one(
        &mut table,
        0x19,
        CSI_INTERMEDIATE_STATE,
        EXECUTE_ACTION,
        CSI_INTERMEDIATE_STATE,
    );
    add_range(
        &mut table,
        0x1C,
        0x1F,
        CSI_INTERMEDIATE_STATE,
        EXECUTE_ACTION,
        CSI_INTERMEDIATE_STATE,
    );
    add_range(
        &mut table,
        0x20,
        0x2F,
        CSI_INTERMEDIATE_STATE,
        COLLECT_ACTION,
        CSI_INTERMEDIATE_STATE,
    );
    add_one(
        &mut table,
        0x7F,
        CSI_INTERMEDIATE_STATE,
        IGNORE_ACTION,
        CSI_INTERMEDIATE_STATE,
    );
    // Csi_intermediate -> Ground
    add_range(
        &mut table,
        0x40,
        0x7E,
        CSI_INTERMEDIATE_STATE,
        DISPATCH_ACTION,
        GROUND_STATE,
    );
    // Csi_intermediate -> Csi_ignore
    add_range(
        &mut table,
        0x30,
        0x3F,
        CSI_INTERMEDIATE_STATE,
        IGNORE_ACTION,
        GROUND_STATE,
    );

    // Csi_entry
    add_range(
        &mut table,
        0x00,
        0x17,
        CSI_ENTRY_STATE,
        EXECUTE_ACTION,
        CSI_ENTRY_STATE,
    );
    add_one(
        &mut table,
        0x19,
        CSI_ENTRY_STATE,
        EXECUTE_ACTION,
        CSI_ENTRY_STATE,
    );
    add_range(
        &mut table,
        0x1C,
        0x1F,
        CSI_ENTRY_STATE,
        EXECUTE_ACTION,
        CSI_ENTRY_STATE,
    );
    add_one(
        &mut table,
        0x7F,
        CSI_ENTRY_STATE,
        IGNORE_ACTION,
        CSI_ENTRY_STATE,
    );
    // Csi_entry -> Ground
    add_range(
        &mut table,
        0x40,
        0x7E,
        CSI_ENTRY_STATE,
        DISPATCH_ACTION,
        GROUND_STATE,
    );
    // Csi_entry -> Csi_intermediate
    add_range(
        &mut table,
        0x20,
        0x2F,
        CSI_ENTRY_STATE,
        COLLECT_ACTION,
        CSI_INTERMEDIATE_STATE,
    );
    // Csi_entry -> Csi_param
    add_range(
        &mut table,
        0x30,
        0x3B,
        CSI_ENTRY_STATE,
        PARAM_ACTION,
        CSI_PARAM_STATE,
    );
    add_range(
        &mut table,
        0x3C,
        0x3F,
        CSI_ENTRY_STATE,
        PREFIX_ACTION,
        CSI_PARAM_STATE,
    );

    // Osc_string
    add_range(
        &mut table,
        0x00,
        0x06,
        OSC_STRING_STATE,
        IGNORE_ACTION,
        OSC_STRING_STATE,
    );
    add_range(
        &mut table,
        0x08,
        0x17,
        OSC_STRING_STATE,
        IGNORE_ACTION,
        OSC_STRING_STATE,
    );
    add_one(
        &mut table,
        0x19,
        OSC_STRING_STATE,
        IGNORE_ACTION,
        OSC_STRING_STATE,
    );
    add_range(
        &mut table,
        0x1C,
        0x1F,
        OSC_STRING_STATE,
        IGNORE_ACTION,
        OSC_STRING_STATE,
    );
    add_range(
        &mut table,
        0x20,
        0xFF,
        OSC_STRING_STATE,
        PUT_ACTION,
        OSC_STRING_STATE,
    );

    // ST, CAN, SUB, ESC, and BEL terminate the sequence
    add_one(
        &mut table,
        0x1B,
        OSC_STRING_STATE,
        DISPATCH_ACTION,
        ESCAPE_STATE,
    );
    add_one(
        &mut table,
        0x07,
        OSC_STRING_STATE,
        DISPATCH_ACTION,
        GROUND_STATE,
    );
    add_one(
        &mut table,
        0x9C,
        OSC_STRING_STATE,
        DISPATCH_ACTION,
        GROUND_STATE,
    );
    add_many(
        &mut table,
        &[0x18, 0x1A],
        OSC_STRING_STATE,
        IGNORE_ACTION,
        GROUND_STATE,
    );

    table
}

fn add_one(
    table: &mut [u8; DEFAULT_TABLE_SIZE],
    code: u8,
    state: State,
    action: Action,
    next: State,
) {
    let idx = (state as usize) << INDEX_STATE_SHIFT | code as usize;
    let value = action << TRANSITION_ACTION_SHIFT | next;
    table[idx] = value;
}

fn add_many(
    table: &mut [u8; DEFAULT_TABLE_SIZE],
    codes: &[u8],
    state: State,
    action: Action,
    next: State,
) {
    for &code in codes {
        add_one(table, code, state, action, next);
    }
}

fn add_range(
    table: &mut [u8; DEFAULT_TABLE_SIZE],
    start: u8,
    end: u8,
    state: State,
    action: Action,
    next: State,
) {
    for code in start..=end {
        add_one(table, code, state, action, next);
    }
}

/// C0 control characters.
///
/// These range from (0x00-0x1F) as defined in ISO 646 (ASCII).
pub const NUL: u8 = 0x00;
/// SOH is the start of heading character.
pub const SOH: u8 = 0x01;
/// STX is the start of text character.
pub const STX: u8 = 0x02;
/// ETX is the end of text character.
pub const ETX: u8 = 0x03;
/// EOT is the end of transmission character.
pub const EOT: u8 = 0x04;
/// ENQ is the enquiry character.
pub const ENQ: u8 = 0x05;
/// ACK is the acknowledge character.
pub const ACK: u8 = 0x06;
/// BEL is the bell character.
pub const BEL: u8 = 0x07;
/// BS is the backspace character.
pub const BS: u8 = 0x08;
/// HT is the horizontal tab character.
pub const HT: u8 = 0x09;
/// LF is the line feed character.
pub const LF: u8 = 0x0A;
/// VT is the vertical tab character.
pub const VT: u8 = 0x0B;
/// FF is the form feed character.
pub const FF: u8 = 0x0C;
/// CR is the carriage return character.
pub const CR: u8 = 0x0D;
/// SO is the shift out character.
pub const SO: u8 = 0x0E;
/// SI is the shift in character.
pub const SI: u8 = 0x0F;
/// DLE is the data link escape character.
pub const DLE: u8 = 0x10;
/// DC1 is the device control 1 character.
pub const DC1: u8 = 0x11;
/// DC2 is the device control 2 character.
pub const DC2: u8 = 0x12;
/// DC3 is the device control 3 character.
pub const DC3: u8 = 0x13;
/// DC4 is the device control 4 character.
pub const DC4: u8 = 0x14;
/// NAK is the negative acknowledge character.
pub const NAK: u8 = 0x15;
/// SYN is the synchronous idle character.
pub const SYN: u8 = 0x16;
/// ETB is the end of transmission block character.
pub const ETB: u8 = 0x17;
/// CAN is the cancel character.
pub const CAN: u8 = 0x18;
/// EM is the end of medium character.
pub const EM: u8 = 0x19;
/// SUB is the substitute character.
pub const SUB: u8 = 0x1A;
/// ESC is the escape character.
pub const ESC: u8 = 0x1B;
/// FS is the file separator character.
pub const FS: u8 = 0x1C;
/// GS is the group separator character.
pub const GS: u8 = 0x1D;
/// RS is the record separator character.
pub const RS: u8 = 0x1E;
/// US is the unit separator character.
pub const US: u8 = 0x1F;

/// LS0 is the locking shift 0 character (alias for [SI]).
pub const LS0: u8 = SI;
/// LS1 is the locking shift 1 character (alias for [SO]).
pub const LS1: u8 = SO;

/// C1 control characters.
///
/// These range from (0x80-0x9F) as defined in ISO 6429 (ECMA-48).
pub const PAD: u8 = 0x80;
/// HOP is the high octet preset character.
pub const HOP: u8 = 0x81;
/// BPH is the break permitted here character.
pub const BPH: u8 = 0x82;
/// NBH is the no break here character.
pub const NBH: u8 = 0x83;
/// IND is the index character.
pub const IND: u8 = 0x84;
/// NEL is the next line character.
pub const NEL: u8 = 0x85;
/// SSA is the start of selected area character.
pub const SSA: u8 = 0x86;
/// ESA is the end of selected area character.
pub const ESA: u8 = 0x87;
/// HTS is the horizontal tab set character.
pub const HTS: u8 = 0x88;
/// HTJ is the horizontal tab with justification character.
pub const HTJ: u8 = 0x89;
/// VTS is the vertical tab set character.
pub const VTS: u8 = 0x8A;
/// PLD is the partial line forward character.
pub const PLD: u8 = 0x8B;
/// PLU is the partial line backward character.
pub const PLU: u8 = 0x8C;
/// RI is the reverse index character.
pub const RI: u8 = 0x8D;
/// SS2 is the single shift 2 character.
pub const SS2: u8 = 0x8E;
/// SS3 is the single shift 3 character.
pub const SS3: u8 = 0x8F;
/// DCS is the device control string character.
pub const DCS: u8 = 0x90;
/// PU1 is the private use 1 character.
pub const PU1: u8 = 0x91;
/// PU2 is the private use 2 character.
pub const PU2: u8 = 0x92;
/// STS is the set transmit state character.
pub const STS: u8 = 0x93;
/// CCH is the cancel character.
pub const CCH: u8 = 0x94;
/// MW is the message waiting character.
pub const MW: u8 = 0x95;
/// SPA is the start of guarded area character.
pub const SPA: u8 = 0x96;
/// EPA is the end of guarded area character.
pub const EPA: u8 = 0x97;
/// SOS is the start of string character.
pub const SOS: u8 = 0x98;
/// SGCI is the single graphic character introducer character.
pub const SGCI: u8 = 0x99;
/// SCI is the single character introducer character.
pub const SCI: u8 = 0x9A;
/// CSI is the control sequence introducer character.
pub const CSI: u8 = 0x9B;
/// ST is the string terminator character.
pub const ST: u8 = 0x9C;
/// OSC is the operating system command character.
pub const OSC: u8 = 0x9D;
/// PM is the privacy message character.
pub const PM: u8 = 0x9E;
/// APC is the application program command character.
pub const APC: u8 = 0x9F;

/// SP is the space character.
pub const SP: u8 = 0x20;
/// DEL is the delete character.
pub const DEL: u8 = 0x7F;

/// Cmd represents a sequence command. This is used to pack/unpack a sequence
/// command with its intermediate and prefix characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cmd(pub i32);

impl Cmd {
    /// Prefix returns the unpacked prefix byte of the sequence.
    pub fn prefix(&self) -> u8 {
        ((self.0 >> PREFIX_SHIFT) & FINAL_MASK) as u8
    }

    /// Intermediate returns the unpacked intermediate byte of the sequence.
    pub fn intermediate(&self) -> u8 {
        ((self.0 >> INTERMED_SHIFT) & FINAL_MASK) as u8
    }

    /// Final returns the unpacked command byte of the sequence.
    pub fn final_(&self) -> u8 {
        (self.0 & FINAL_MASK) as u8
    }
}

/// Command packs a command with the given prefix, intermediate, and final.
/// A zero byte means the sequence does not have a prefix or intermediate.
pub fn command(prefix: u8, inter: u8, final_: u8) -> i32 {
    let mut c = final_ as i32;
    c |= (prefix as i32) << PREFIX_SHIFT;
    c |= (inter as i32) << INTERMED_SHIFT;
    c
}

/// Param represents a sequence parameter. Sequence parameters with
/// sub-parameters are packed with the [HAS_MORE_FLAG] set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Param(pub i32);

impl Param {
    /// Param returns the unpacked parameter. It returns the default value if
    /// the parameter is missing.
    pub fn param(&self, def: i32) -> i32 {
        let p = self.0 & PARAM_MASK;
        if p == MISSING_PARAM {
            return def;
        }
        p
    }

    /// HasMore unpacks the [HAS_MORE_FLAG] from the parameter.
    pub fn has_more(&self) -> bool {
        self.0 & HAS_MORE_FLAG != 0
    }
}

/// Parameter packs an escape code parameter with the given parameter and
/// whether this parameter has following sub-parameters.
pub fn parameter(p: i32, has_more: bool) -> i32 {
    let mut s = p & PARAM_MASK;
    if has_more {
        s |= HAS_MORE_FLAG;
    }
    s
}

/// Params represents a list of packed parameters.
#[derive(Debug, Clone, Copy)]
pub struct Params<'a>(pub &'a [i32]);

impl<'a> Params<'a> {
    /// Param returns the parameter at the given index. It falls back to the
    /// default value if the parameter is missing. If the index is out of
    /// bounds, it returns the default value and `ok == false`.
    pub fn param(&self, i: usize, def: i32) -> (i32, bool, bool) {
        match self.0.get(i) {
            None => (def, false, false),
            Some(p) => (Param(*p).param(def), Param(*p).has_more(), true),
        }
    }

    /// ForEach iterates over the parameters and calls the given function for
    /// each parameter.
    pub fn for_each(&self, def: i32, f: impl FnMut(usize, i32, bool)) {
        let mut f = f;
        for (i, p) in self.0.iter().enumerate() {
            f(i, Param(*p).param(def), Param(*p).has_more());
        }
    }

    /// Returns the raw packed parameter slice.
    pub fn as_slice(&self) -> &'a [i32] {
        self.0
    }
}

/// Handler handles actions performed by the parser.
/// It is used to handle ANSI escape sequences, control characters, and runes.
///
/// The handler callback types mirror the upstream Go `Handler` struct's
/// fields (e.g. `CsiHandler func(cmd Cmd, params []int)`).
type CsiHandler<'a> = Option<&'a mut dyn FnMut(Cmd, &[i32])>;
type DcsHandler<'a> = Option<&'a mut dyn FnMut(Cmd, &[i32], &[u8])>;
type OscHandler<'a> = Option<&'a mut dyn FnMut(i32, &[u8])>;
type BytesHandler<'a> = Option<&'a mut dyn FnMut(&[u8])>;

#[derive(Default)]
pub struct Handler<'a> {
    /// Print is called when a printable rune is encountered.
    pub print: Option<&'a mut dyn FnMut(char)>,
    /// Execute is called when a control character is encountered.
    pub execute: Option<&'a mut dyn FnMut(u8)>,
    /// HandleCsi is called when a CSI sequence is encountered.
    pub handle_csi: CsiHandler<'a>,
    /// HandleEsc is called when an ESC sequence is encountered.
    pub handle_esc: Option<&'a mut dyn FnMut(Cmd)>,
    /// HandleDcs is called when a DCS sequence is encountered.
    pub handle_dcs: DcsHandler<'a>,
    /// HandleOsc is called when an OSC sequence is encountered.
    pub handle_osc: OscHandler<'a>,
    /// HandlePm is called when a PM sequence is encountered.
    pub handle_pm: BytesHandler<'a>,
    /// HandleApc is called when an APC sequence is encountered.
    pub handle_apc: BytesHandler<'a>,
    /// HandleSos is called when a SOS sequence is encountered.
    pub handle_sos: BytesHandler<'a>,
}

/// Parser represents a DEC ANSI compatible sequence parser.
///
/// It uses a state machine to parse ANSI escape sequences and control
/// characters.
pub struct Parser<'a> {
    handler: Handler<'a>,

    /// params contains the raw parameters of the sequence.
    params: Vec<i32>,

    /// data contains the raw data of the sequence.
    data: Vec<u8>,

    /// dataLen keeps track of the length of the data buffer. If -1, the data
    /// buffer is unlimited and will grow as needed.
    data_len: i32,

    /// paramsLen keeps track of the number of parameters. This is also used
    /// when collecting UTF-8 runes to keep track of the number of rune bytes
    /// collected.
    params_len: usize,

    /// cmd contains the raw command along with the private prefix and
    /// intermediate bytes of the sequence. This is also used when collecting
    /// UTF-8 runes treating it as a slice of 4 bytes.
    cmd: i32,

    /// state is the current state of the parser.
    state: State,
}

/// NewParser returns a new parser with the default settings. The [Parser]
/// uses a default size of 32 for the parameters and 64KB for the data buffer.
pub fn new_parser() -> Parser<'static> {
    Parser {
        handler: Handler::default(),
        params: vec![MISSING_PARAM; MAX_PARAMS_SIZE],
        data: vec![0; 1024 * 64],
        data_len: 0,
        params_len: 0,
        cmd: 0,
        state: GROUND_STATE,
    }
}

/// GetParser returns a parser from a sync pool.
///
/// NOTE: upstream uses a `sync.Pool`; the pool is a performance optimization
/// with no observable semantics, so we return a fresh parser instead.
pub fn get_parser() -> Parser<'static> {
    let mut p = new_parser();
    p.set_params_size(MAX_PARAMS_SIZE);
    p.set_data_size(1024 * 4);
    p
}

/// PutParser returns a parser to a sync pool. The parser is reset
/// automatically.
pub fn put_parser(p: &mut Parser) {
    p.reset();
    p.data_len = 0;
}

impl<'a> Parser<'a> {
    /// NewParserWithHandler returns a new parser with the given handler.
    pub fn with_handler(handler: Handler<'a>) -> Parser<'a> {
        Parser {
            handler,
            params: vec![MISSING_PARAM; MAX_PARAMS_SIZE],
            data: vec![0; 1024 * 64],
            data_len: 0,
            params_len: 0,
            cmd: 0,
            state: GROUND_STATE,
        }
    }

    /// SetHandler sets the handler for the parser.
    pub fn set_handler(&mut self, handler: Handler<'a>) {
        self.handler = handler;
    }

    /// SetParamsSize sets the size of the parameters buffer.
    pub fn set_params_size(&mut self, size: usize) {
        self.params = vec![MISSING_PARAM; size];
    }

    /// SetDataSize sets the size of the data buffer. If size is less than or
    /// equal to 0, the data buffer is unlimited and will grow as needed.
    pub fn set_data_size(&mut self, size: usize) {
        if size == 0 {
            self.data = Vec::new();
            self.data_len = -1;
        } else {
            self.data = vec![0; size];
            self.data_len = 0;
        }
    }

    /// Params returns the list of parsed packed parameters.
    pub fn params(&self) -> Params<'_> {
        Params(&self.params[..self.params_len])
    }

    /// Param returns the parameter at the given index and falls back to the
    /// default value if the parameter is missing. If the index is out of
    /// bounds, it returns the default value and `ok == false`.
    pub fn param(&self, i: usize, def: i32) -> (i32, bool) {
        if i >= self.params_len {
            return (def, false);
        }
        (Param(self.params[i]).param(def), true)
    }

    /// Command returns the packed command of the last dispatched sequence.
    pub fn command(&self) -> i32 {
        self.cmd
    }

    /// Rune returns the last dispatched sequence as a rune.
    pub fn rune(&self) -> char {
        let b0 = (self.cmd & 0xff) as u8;
        let rw = utf8_byte_len(b0);
        if rw == -1 {
            return '\u{FFFD}';
        }
        let mut bytes = [0u8; 4];
        for (i, b) in bytes.iter_mut().take(rw as usize).enumerate() {
            *b = ((self.cmd >> (i * 8)) & 0xff) as u8;
        }
        let s = std::str::from_utf8(&bytes[..rw as usize]).unwrap_or("\u{FFFD}");
        s.chars().next().unwrap_or('\u{FFFD}')
    }

    /// Control returns the last dispatched sequence as a control code.
    pub fn control(&self) -> u8 {
        (self.cmd & 0xff) as u8
    }

    /// Data returns the raw data of the last dispatched sequence.
    pub fn data(&self) -> &[u8] {
        let len = if self.data_len >= 0 {
            self.data_len as usize
        } else {
            self.data.len()
        };
        &self.data[..len]
    }

    /// Reset resets the parser to its initial state.
    pub fn reset(&mut self) {
        self.clear();
        self.state = GROUND_STATE;
    }

    /// clear clears the parser parameters and command.
    fn clear(&mut self) {
        if !self.params.is_empty() {
            self.params[0] = MISSING_PARAM;
        }
        self.params_len = 0;
        self.cmd = 0;
    }

    /// State returns the current state of the parser.
    pub fn state(&self) -> State {
        self.state
    }

    /// StateName returns the name of the current state.
    pub fn state_name(&self) -> &'static str {
        state_name(self.state)
    }

    /// Parse parses the given byte buffer.
    pub fn parse(&mut self, b: &[u8]) {
        for &byte in b {
            self.advance(byte);
        }
    }

    /// Advance advances the parser using the given byte. It returns the
    /// action performed by the parser.
    pub fn advance(&mut self, b: u8) -> Action {
        if self.state == UTF8_STATE {
            return self.advance_utf8(b);
        }
        self.advance_byte(b)
    }

    fn collect_rune(&mut self, b: u8) {
        if self.params_len >= 4 {
            return;
        }
        let shift = self.params_len * 8;
        self.cmd &= !(0xff << shift);
        self.cmd |= (b as i32) << shift;
        self.params_len += 1;
    }

    fn advance_utf8(&mut self, b: u8) -> Action {
        // Collect UTF-8 rune bytes.
        self.collect_rune(b);
        let rw = utf8_byte_len((self.cmd & 0xff) as u8);
        if rw == -1 {
            // This panics upstream because the first byte comes from the
            // state machine; mirror it.
            panic!("invalid rune");
        }

        if self.params_len < rw as usize {
            return COLLECT_ACTION;
        }

        // We have enough bytes to decode the rune.
        let r = self.rune();
        if let Some(print) = &mut self.handler.print {
            print(r);
        }

        self.state = GROUND_STATE;
        self.params_len = 0;

        PRINT_ACTION
    }

    fn advance_byte(&mut self, b: u8) -> Action {
        let (state, action) = transition(self.state, b);

        // We need to clear the parser state if the state changes from
        // EscapeState.
        if self.state != state {
            if self.state == ESCAPE_STATE {
                self.perform_action(CLEAR_ACTION, state, b);
            }
            if action == PUT_ACTION && self.state == DCS_ENTRY_STATE && state == DCS_STRING_STATE {
                self.perform_action(START_ACTION, state, 0);
            }
        }

        // Handle special cases
        if b == ESC && self.state == ESCAPE_STATE {
            // Two ESCs in a row
            self.perform_action(EXECUTE_ACTION, state, b);
        } else {
            self.perform_action(action, state, b);
        }

        self.state = state;

        action
    }

    fn parse_string_cmd(&mut self) {
        // Try to parse the command
        let datalen = if self.data_len >= 0 {
            self.data_len as usize
        } else {
            self.data.len()
        };
        for i in 0..datalen {
            let d = self.data[i];
            if !d.is_ascii_digit() {
                break;
            }
            if self.cmd == MISSING_COMMAND {
                self.cmd = 0;
            }
            self.cmd *= 10;
            self.cmd += (d - b'0') as i32;
        }
    }

    fn perform_action(&mut self, action: Action, state: State, b: u8) {
        match action {
            IGNORE_ACTION => {}

            CLEAR_ACTION => {
                self.clear();
            }

            PRINT_ACTION => {
                self.cmd = b as i32;
                if let Some(print) = &mut self.handler.print {
                    print(b as char);
                }
            }

            EXECUTE_ACTION => {
                self.cmd = b as i32;
                if let Some(execute) = &mut self.handler.execute {
                    execute(b);
                }
            }

            PREFIX_ACTION => {
                // Collect private prefix; we only store the last prefix.
                self.cmd &= !(0xff << PREFIX_SHIFT);
                self.cmd |= (b as i32) << PREFIX_SHIFT;
            }

            COLLECT_ACTION => {
                if state == UTF8_STATE {
                    // Reset the UTF-8 counter
                    self.params_len = 0;
                    self.collect_rune(b);
                } else {
                    // Collect intermediate bytes; we only store the last
                    // intermediate byte.
                    self.cmd &= !(0xff << INTERMED_SHIFT);
                    self.cmd |= (b as i32) << INTERMED_SHIFT;
                }
            }

            PARAM_ACTION => {
                // Collect parameters
                if self.params_len >= self.params.len() {
                    return;
                }

                if b.is_ascii_digit() {
                    if self.params[self.params_len] == MISSING_PARAM {
                        self.params[self.params_len] = 0;
                    }
                    self.params[self.params_len] *= 10;
                    self.params[self.params_len] += (b - b'0') as i32;
                }

                if b == b':' {
                    self.params[self.params_len] |= HAS_MORE_FLAG;
                }

                if b == b';' || b == b':' {
                    self.params_len += 1;
                    if self.params_len < self.params.len() {
                        self.params[self.params_len] = MISSING_PARAM;
                    }
                }
            }

            START_ACTION => {
                if self.data_len < 0 && !self.data.is_empty() {
                    self.data = Vec::new();
                } else {
                    self.data_len = 0;
                }
                if (DCS_ENTRY_STATE..=DCS_STRING_STATE).contains(&state) {
                    // Collect the command byte for DCS
                    self.cmd |= b as i32;
                } else {
                    self.cmd = MISSING_COMMAND;
                }
            }

            PUT_ACTION => {
                if self.state == OSC_STRING_STATE && b == b';' && self.cmd == MISSING_COMMAND {
                    self.parse_string_cmd();
                }

                if self.data_len < 0 {
                    self.data.push(b);
                } else if (self.data_len as usize) < self.data.len() {
                    self.data[self.data_len as usize] = b;
                    self.data_len += 1;
                }
            }

            DISPATCH_ACTION => {
                // Increment the last parameter
                if self.params_len > 0 && self.params_len < self.params.len() - 1
                    || self.params_len == 0
                        && !self.params.is_empty()
                        && self.params[0] != MISSING_PARAM
                {
                    self.params_len += 1;
                }

                if self.state == OSC_STRING_STATE && self.cmd == MISSING_COMMAND {
                    // Ensure we have a command for OSC
                    self.parse_string_cmd();
                }

                let data_len = if self.data_len >= 0 {
                    self.data_len as usize
                } else {
                    self.data.len()
                };
                let state = self.state;
                match state {
                    CSI_ENTRY_STATE | CSI_PARAM_STATE | CSI_INTERMEDIATE_STATE => {
                        self.cmd |= b as i32;
                        let cmd = Cmd(self.cmd);
                        let params = Params(&self.params[..self.params_len]);
                        if let Some(handle_csi) = &mut self.handler.handle_csi {
                            handle_csi(cmd, params.as_slice());
                        }
                    }
                    ESCAPE_STATE | ESCAPE_INTERMEDIATE_STATE => {
                        self.cmd |= b as i32;
                        let cmd = Cmd(self.cmd);
                        if let Some(handle_esc) = &mut self.handler.handle_esc {
                            handle_esc(cmd);
                        }
                    }
                    DCS_ENTRY_STATE
                    | DCS_PARAM_STATE
                    | DCS_INTERMEDIATE_STATE
                    | DCS_STRING_STATE => {
                        let cmd = Cmd(self.cmd);
                        let params = Params(&self.params[..self.params_len]);
                        let data = &self.data[..data_len];
                        if let Some(handle_dcs) = &mut self.handler.handle_dcs {
                            handle_dcs(cmd, params.as_slice(), data);
                        }
                    }
                    OSC_STRING_STATE => {
                        let cmd = self.cmd;
                        let data = &self.data[..data_len];
                        if let Some(handle_osc) = &mut self.handler.handle_osc {
                            handle_osc(cmd, data);
                        }
                    }
                    SOS_STRING_STATE => {
                        let data = &self.data[..data_len];
                        if let Some(handle_sos) = &mut self.handler.handle_sos {
                            handle_sos(data);
                        }
                    }
                    PM_STRING_STATE => {
                        let data = &self.data[..data_len];
                        if let Some(handle_pm) = &mut self.handler.handle_pm {
                            handle_pm(data);
                        }
                    }
                    APC_STRING_STATE => {
                        let data = &self.data[..data_len];
                        if let Some(handle_apc) = &mut self.handler.handle_apc {
                            handle_apc(data);
                        }
                    }
                    _ => {}
                }
            }

            _ => {}
        }
    }
}

fn utf8_byte_len(b: u8) -> i32 {
    if b <= 0b0111_1111 {
        1
    } else if (0b1100_0000..=0b1101_1111).contains(&b) {
        2
    } else if (0b1110_0000..=0b1110_1111).contains(&b) {
        3
    } else if (0b1111_0000..=0b1111_0111).contains(&b) {
        4
    } else {
        -1
    }
}

/// State represents the state of the ANSI escape sequence parser used by
/// [decode_sequence].
pub type DecodeState = u8;

/// ANSI escape sequence states used by [decode_sequence].
pub const NORMAL_STATE: DecodeState = 0;
/// PrefixState collects the private prefix.
pub const PREFIX_STATE: DecodeState = 1;
/// ParamsState collects parameters.
pub const PARAMS_STATE: DecodeState = 2;
/// IntermedState collects intermediate bytes.
pub const INTERMED_STATE: DecodeState = 3;
/// EscapeState is the ESC state.
pub const ESCAPE_STATE_DECODE: DecodeState = 4;
/// StringState collects string data.
pub const STRING_STATE: DecodeState = 5;

/// A sequence decoded by [decode_sequence]: the sequence slice, its cell
/// width, the number of bytes consumed, and the new decoder state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decoded<'a> {
    /// The decoded sequence slice.
    pub seq: &'a [u8],
    /// The cell width of the sequence. Always 0 for control and escape
    /// sequences, 1 for ASCII printable characters, and the number of cells
    /// other Unicode characters occupy.
    pub width: usize,
    /// The number of bytes read.
    pub n: usize,
    /// The new decoder state.
    pub state: DecodeState,
}

/// DecodeSequence decodes the first ANSI escape sequence or a printable
/// grapheme from the given data.
///
/// The cell width will always be 0 for control and escape sequences, 1 for
/// ASCII printable characters, and the number of cells other Unicode
/// characters occupy. It always does grapheme clustering (mode 2027).
///
/// Passing a non-None parser as the last argument will allow the decoder to
/// collect sequence parameters, data, and commands. Zero [Cmd] means the CSI,
/// DCS, or ESC sequence is invalid.
pub fn decode_sequence<'a>(b: &'a [u8], state: DecodeState, p: Option<&mut Parser>) -> Decoded<'a> {
    decode_sequence_method(crate::method::WidthMethod::GraphemeWidth, b, state, p)
}

/// DecodeSequenceWc decodes the first ANSI escape sequence or a printable
/// grapheme from the given data, treating the text as a sequence of wide
/// characters and runes.
pub fn decode_sequence_wc<'a>(
    b: &'a [u8],
    state: DecodeState,
    p: Option<&mut Parser>,
) -> Decoded<'a> {
    decode_sequence_method(crate::method::WidthMethod::WcWidth, b, state, p)
}

fn decode_sequence_method<'a>(
    m: crate::method::WidthMethod,
    b: &'a [u8],
    mut state: DecodeState,
    mut p: Option<&mut Parser>,
) -> Decoded<'a> {
    for i in 0..b.len() {
        let c = b[i];

        // The prefix/params/intermed states fall through to the next state
        // with the same byte; loop until the byte is fully consumed.
        'byte: loop {
            match state {
                NORMAL_STATE => {
                    match c {
                        ESC => {
                            if let Some(p) = &mut p {
                                if !p.params.is_empty() {
                                    p.params[0] = MISSING_PARAM;
                                }
                                p.cmd = 0;
                                p.params_len = 0;
                                p.data_len = 0;
                            }
                            state = ESCAPE_STATE_DECODE;
                            break 'byte;
                        }
                        CSI | DCS => {
                            if let Some(p) = &mut p {
                                if !p.params.is_empty() {
                                    p.params[0] = MISSING_PARAM;
                                }
                                p.cmd = 0;
                                p.params_len = 0;
                                p.data_len = 0;
                            }
                            state = PREFIX_STATE;
                            break 'byte;
                        }
                        OSC | APC | SOS | PM => {
                            if let Some(p) = &mut p {
                                p.cmd = MISSING_COMMAND;
                                p.data_len = 0;
                            }
                            state = STRING_STATE;
                            break 'byte;
                        }
                        _ => {}
                    }

                    if let Some(p) = &mut p {
                        p.data_len = 0;
                        p.params_len = 0;
                        p.cmd = 0;
                    }
                    if c > US && c < DEL {
                        // ASCII printable characters
                        return Decoded {
                            seq: &b[i..i + 1],
                            width: 1,
                            n: 1,
                            state: NORMAL_STATE,
                        };
                    }

                    if c <= US || c == DEL || c < 0xC0 {
                        // C0 & C1 control characters & DEL
                        return Decoded {
                            seq: &b[i..i + 1],
                            width: 0,
                            n: 1,
                            state: NORMAL_STATE,
                        };
                    }

                    if (0x80..=0xF7).contains(&c) {
                        let (cluster, width) = first_grapheme_cluster(&b[i..], m);
                        let n = i + cluster.len();
                        return Decoded {
                            seq: &b[..n],
                            width,
                            n,
                            state: NORMAL_STATE,
                        };
                    }

                    // Invalid UTF-8 sequence
                    return Decoded {
                        seq: &b[..i],
                        width: 0,
                        n: i,
                        state: NORMAL_STATE,
                    };
                }
                PREFIX_STATE => {
                    if (b'<'..=b'?').contains(&c) {
                        if let Some(p) = &mut p {
                            // We only collect the last prefix character.
                            p.cmd &= !(0xff << PREFIX_SHIFT);
                            p.cmd |= (c as i32) << PREFIX_SHIFT;
                        }
                        break 'byte;
                    }

                    state = PARAMS_STATE;
                    continue 'byte;
                }
                PARAMS_STATE => {
                    if c.is_ascii_digit() {
                        if let Some(p) = &mut p {
                            if p.params[p.params_len] == MISSING_PARAM {
                                p.params[p.params_len] = 0;
                            }
                            p.params[p.params_len] *= 10;
                            p.params[p.params_len] += (c - b'0') as i32;
                        }
                        break 'byte;
                    }

                    if c == b':' {
                        if let Some(p) = &mut p {
                            p.params[p.params_len] |= HAS_MORE_FLAG;
                        }
                    }

                    if c == b';' || c == b':' {
                        if let Some(p) = &mut p {
                            p.params_len += 1;
                            if p.params_len < p.params.len() {
                                p.params[p.params_len] = MISSING_PARAM;
                            }
                        }
                        break 'byte;
                    }

                    state = INTERMED_STATE;
                    continue 'byte;
                }
                INTERMED_STATE => {
                    if (b' '..=b'/').contains(&c) {
                        if let Some(p) = &mut p {
                            p.cmd &= !(0xff << INTERMED_SHIFT);
                            p.cmd |= (c as i32) << INTERMED_SHIFT;
                        }
                        break 'byte;
                    }

                    if let Some(p) = &mut p {
                        // Increment the last parameter
                        if p.params_len > 0 && p.params_len < p.params.len() - 1
                            || p.params_len == 0
                                && !p.params.is_empty()
                                && p.params[0] != MISSING_PARAM
                        {
                            p.params_len += 1;
                        }
                    }

                    if (b'@'..=b'~').contains(&c) {
                        if let Some(p) = &mut p {
                            p.cmd &= !0xff;
                            p.cmd |= c as i32;
                        }

                        if has_dcs_prefix(b) {
                            // Continue to collect DCS data
                            if let Some(p) = &mut p {
                                p.data_len = 0;
                            }
                            state = STRING_STATE;
                            break 'byte;
                        }

                        return Decoded {
                            seq: &b[..i + 1],
                            width: 0,
                            n: i + 1,
                            state: NORMAL_STATE,
                        };
                    }

                    // Invalid CSI/DCS sequence
                    return Decoded {
                        seq: &b[..i],
                        width: 0,
                        n: i,
                        state: NORMAL_STATE,
                    };
                }
                ESCAPE_STATE_DECODE => {
                    match c {
                        b'[' | b'P' => {
                            if let Some(p) = &mut p {
                                if !p.params.is_empty() {
                                    p.params[0] = MISSING_PARAM;
                                }
                                p.params_len = 0;
                                p.cmd = 0;
                            }
                            state = PREFIX_STATE;
                            break 'byte;
                        }
                        b']' | b'X' | b'^' | b'_' => {
                            if let Some(p) = &mut p {
                                p.cmd = MISSING_COMMAND;
                                p.data_len = 0;
                            }
                            state = STRING_STATE;
                            break 'byte;
                        }
                        _ => {}
                    }

                    if (b' '..=b'/').contains(&c) {
                        if let Some(p) = &mut p {
                            p.cmd &= !(0xff << INTERMED_SHIFT);
                            p.cmd |= (c as i32) << INTERMED_SHIFT;
                        }
                        break 'byte;
                    } else if (b'0'..=b'~').contains(&c) {
                        if let Some(p) = &mut p {
                            p.cmd &= !0xff;
                            p.cmd |= c as i32;
                        }
                        return Decoded {
                            seq: &b[..i + 1],
                            width: 0,
                            n: i + 1,
                            state: NORMAL_STATE,
                        };
                    }

                    // Invalid escape sequence
                    return Decoded {
                        seq: &b[..i],
                        width: 0,
                        n: i,
                        state: NORMAL_STATE,
                    };
                }
                STRING_STATE => {
                    match c {
                        BEL => {
                            if has_osc_prefix(b) {
                                parse_osc_cmd(p.as_deref_mut());
                                return Decoded {
                                    seq: &b[..i + 1],
                                    width: 0,
                                    n: i + 1,
                                    state: NORMAL_STATE,
                                };
                            }
                        }
                        CAN | SUB => {
                            if has_osc_prefix(b) {
                                // Ensure we parse the OSC command number
                                parse_osc_cmd(p.as_deref_mut());
                            }

                            // Cancel the sequence
                            return Decoded {
                                seq: &b[..i],
                                width: 0,
                                n: i,
                                state: NORMAL_STATE,
                            };
                        }
                        ST => {
                            if has_osc_prefix(b) {
                                // Ensure we parse the OSC command number
                                parse_osc_cmd(p.as_deref_mut());
                            }

                            return Decoded {
                                seq: &b[..i + 1],
                                width: 0,
                                n: i + 1,
                                state: NORMAL_STATE,
                            };
                        }
                        ESC => {
                            if has_st_prefix(&b[i..]) {
                                if has_osc_prefix(b) {
                                    // Ensure we parse the OSC command number
                                    parse_osc_cmd(p.as_deref_mut());
                                }

                                // End of string 7-bit (ST)
                                return Decoded {
                                    seq: &b[..i + 2],
                                    width: 0,
                                    n: i + 2,
                                    state: NORMAL_STATE,
                                };
                            }

                            // Otherwise, cancel the sequence
                            return Decoded {
                                seq: &b[..i],
                                width: 0,
                                n: i,
                                state: NORMAL_STATE,
                            };
                        }
                        _ => {}
                    }

                    if let Some(p) = &mut p {
                        if p.data_len < 0 || (p.data_len as usize) < p.data.len() {
                            p.data[p.data_len.max(0) as usize] = c;
                            p.data_len += 1;

                            // Parse the OSC command number
                            if c == b';' && has_osc_prefix(b) {
                                parse_osc_cmd(Some(&mut *p));
                            }
                        }
                    }
                    break 'byte;
                }
                _ => break 'byte,
            }
        }
    }

    Decoded {
        seq: b,
        width: 0,
        n: b.len(),
        state,
    }
}

fn parse_osc_cmd(p: Option<&mut Parser>) {
    if let Some(p) = p {
        if p.cmd == MISSING_COMMAND {
            for j in 0..p.data_len.max(0) as usize {
                let d = p.data[j];
                if !d.is_ascii_digit() {
                    break;
                }
                if p.cmd == MISSING_COMMAND {
                    p.cmd = 0;
                }
                p.cmd *= 10;
                p.cmd += (d - b'0') as i32;
            }
        }
    }
}

/// FirstGraphemeCluster returns the first grapheme cluster in the given byte
/// slice and its monospace display width.
pub fn first_grapheme_cluster(b: &[u8], m: crate::method::WidthMethod) -> (&[u8], usize) {
    let s = std::str::from_utf8(b).unwrap_or("\u{FFFD}");
    let cluster: &str = crate::width::first_grapheme_cluster(s).unwrap_or(s);
    let width = match m {
        crate::method::WidthMethod::WcWidth => crate::width::string_width_wc(cluster),
        crate::method::WidthMethod::GraphemeWidth => crate::width::string_width(cluster),
    };
    (&b[..cluster.len()], width)
}

/// HasCsiPrefix returns true if the given byte slice has a CSI prefix.
pub fn has_csi_prefix(b: &[u8]) -> bool {
    (!b.is_empty() && b[0] == CSI) || (b.len() > 1 && b[0] == ESC && b[1] == b'[')
}

/// HasOscPrefix returns true if the given byte slice has an OSC prefix.
pub fn has_osc_prefix(b: &[u8]) -> bool {
    (!b.is_empty() && b[0] == OSC) || (b.len() > 1 && b[0] == ESC && b[1] == b']')
}

/// HasApcPrefix returns true if the given byte slice has an APC prefix.
pub fn has_apc_prefix(b: &[u8]) -> bool {
    (!b.is_empty() && b[0] == APC) || (b.len() > 1 && b[0] == ESC && b[1] == b'_')
}

/// HasDcsPrefix returns true if the given byte slice has a DCS prefix.
pub fn has_dcs_prefix(b: &[u8]) -> bool {
    (!b.is_empty() && b[0] == DCS) || (b.len() > 1 && b[0] == ESC && b[1] == b'P')
}

/// HasSosPrefix returns true if the given byte slice has a SOS prefix.
pub fn has_sos_prefix(b: &[u8]) -> bool {
    (!b.is_empty() && b[0] == SOS) || (b.len() > 1 && b[0] == ESC && b[1] == b'X')
}

/// HasPmPrefix returns true if the given byte slice has a PM prefix.
pub fn has_pm_prefix(b: &[u8]) -> bool {
    (!b.is_empty() && b[0] == PM) || (b.len() > 1 && b[0] == ESC && b[1] == b'^')
}

/// HasStPrefix returns true if the given byte slice has a ST prefix.
pub fn has_st_prefix(b: &[u8]) -> bool {
    (!b.is_empty() && b[0] == ST) || (b.len() > 1 && b[0] == ESC && b[1] == b'\\')
}

/// HasEscPrefix returns true if the given byte slice has an ESC prefix.
pub fn has_esc_prefix(b: &[u8]) -> bool {
    !b.is_empty() && b[0] == ESC
}

/// Execute is a function that "executes" the given escape sequence by writing
/// it to the provided output writer.
pub fn execute(w: &mut dyn Write, s: &str) -> io::Result<usize> {
    w.write(s.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode_all(input: &[u8]) -> Vec<(Vec<u8>, usize, u8)> {
        let mut out = Vec::new();
        let mut state = NORMAL_STATE;
        let mut rest = input;
        while !rest.is_empty() {
            let d = decode_sequence(rest, state, None);
            out.push((d.seq.to_vec(), d.width, d.state));
            state = d.state;
            rest = &rest[d.n..];
        }
        out
    }

    #[test]
    fn test_decode_ascii() {
        let d = decode_all(b"Hello");
        let strings: Vec<&str> = d
            .iter()
            .map(|(s, _, _)| std::str::from_utf8(s).unwrap())
            .collect();
        assert_eq!(strings, vec!["H", "e", "l", "l", "o"]);
        assert!(d.iter().all(|(_, w, _)| *w == 1));
    }

    #[test]
    fn test_decode_csi_sgr() {
        let input = b"\x1b[31;42m";
        let d = decode_sequence(input, NORMAL_STATE, None);
        assert_eq!(d.seq, input);
        assert_eq!(d.width, 0);
        assert_eq!(d.n, input.len());
        assert_eq!(d.state, NORMAL_STATE);
    }

    #[test]
    fn test_decode_csi_with_prefix_and_params() {
        let input = b"\x1b[?1000;1002h";
        let mut p = new_parser();
        let d = decode_sequence(input, NORMAL_STATE, Some(&mut p));
        assert_eq!(d.n, input.len());
        let cmd = Cmd(p.command());
        assert_eq!(cmd.prefix(), b'?');
        assert_eq!(cmd.final_(), b'h');
        let (first, has_more, ok) = p.params().param(0, 0);
        assert!(ok);
        assert_eq!(first, 1000);
        assert!(!has_more);
        let (second, _, _) = p.params().param(1, 0);
        assert_eq!(second, 1002);
    }

    #[test]
    fn test_decode_sub_params() {
        let input = b"\x1b[1:2;3m";
        let mut p = new_parser();
        let d = decode_sequence(input, NORMAL_STATE, Some(&mut p));
        assert_eq!(d.n, input.len());
        let (first, has_more, _) = p.params().param(0, 0);
        assert_eq!(first, 1);
        assert!(has_more);
        let (second, has_more, _) = p.params().param(1, 0);
        assert_eq!(second, 2);
        assert!(!has_more);
        let (third, _, _) = p.params().param(2, 0);
        assert_eq!(third, 3);
    }

    #[test]
    fn test_decode_osc_bel() {
        let input = b"\x1b]0;title\x07";
        let mut p = new_parser();
        let d = decode_sequence(input, NORMAL_STATE, Some(&mut p));
        assert_eq!(d.n, input.len());
        assert_eq!(p.cmd, 0);
        assert_eq!(p.data(), b"0;title");
    }

    #[test]
    fn test_decode_osc_cmd_number() {
        let input = b"\x1b]1337;SetUserVar=1\x07";
        let mut p = new_parser();
        let d = decode_sequence(input, NORMAL_STATE, Some(&mut p));
        assert_eq!(d.n, input.len());
        assert_eq!(p.cmd, 1337);
    }

    #[test]
    fn test_decode_osc_st_two_byte() {
        let input = b"\x1b]52;c\x1b\\";
        let mut p = new_parser();
        let d = decode_sequence(input, NORMAL_STATE, Some(&mut p));
        assert_eq!(d.n, input.len());
        assert_eq!(p.cmd, 52);
        assert_eq!(p.data(), b"52;c");
    }

    #[test]
    fn test_decode_dcs() {
        let input = b"\x1bP1;2|data\x1b\\";
        let mut p = new_parser();
        let d = decode_sequence(input, NORMAL_STATE, Some(&mut p));
        assert_eq!(d.n, input.len());
        assert_eq!(p.cmd & 0xff, b'|' as i32);
        assert_eq!(p.data(), b"data");
    }

    #[test]
    fn test_decode_esc_simple() {
        let input = b"\x1b7";
        let d = decode_sequence(input, NORMAL_STATE, None);
        assert_eq!(d.seq, input);
        assert_eq!(d.width, 0);
    }

    #[test]
    fn test_decode_utf8_grapheme() {
        let input = "界".as_bytes();
        let d = decode_sequence(input, NORMAL_STATE, None);
        assert_eq!(d.seq, input);
        assert_eq!(d.width, 2);
    }

    #[test]
    fn test_decode_control() {
        let d = decode_sequence(b"\x05", NORMAL_STATE, None);
        assert_eq!(d.width, 0);
        assert_eq!(d.n, 1);
    }

    #[test]
    fn test_decode_continuation_state() {
        // Split sequence across two calls using the returned state.
        let part1 = b"\x1b[31";
        let d1 = decode_sequence(part1, NORMAL_STATE, None);
        assert_eq!(d1.state, PARAMS_STATE);
        let part2 = b"m";
        let d2 = decode_sequence(part2, d1.state, None);
        assert_eq!(d2.state, NORMAL_STATE);
        assert_eq!(d2.n, 1);
    }

    #[test]
    fn test_transition_ground_print() {
        assert_eq!(transition(GROUND_STATE, b'A'), (GROUND_STATE, PRINT_ACTION));
        assert_eq!(transition(GROUND_STATE, ESC), (ESCAPE_STATE, CLEAR_ACTION));
        assert_eq!(
            transition(ESCAPE_STATE, b'['),
            (CSI_ENTRY_STATE, CLEAR_ACTION)
        );
    }

    #[test]
    fn test_parser_state_machine_csi() {
        let mut events = Vec::new();
        let mut csi = |cmd: Cmd, params: &[i32]| {
            events.push(format!("csi:{:?}:{}", cmd.0, params.len()));
        };
        let handler = Handler {
            handle_csi: Some(&mut csi),
            ..Default::default()
        };
        let mut p = Parser::with_handler(handler);
        p.parse(b"\x1b[10;20H");
        drop(p);
        assert_eq!(events.len(), 1);
        assert!(events[0].starts_with("csi:"));
    }

    #[test]
    fn test_parser_state_machine_print_and_execute() {
        let mut printed = String::new();
        let mut executed = Vec::new();
        let mut print = |r: char| printed.push(r);
        let mut exec = |b: u8| executed.push(b);
        let handler = Handler {
            print: Some(&mut print),
            execute: Some(&mut exec),
            ..Default::default()
        };
        let mut p = Parser::with_handler(handler);
        p.parse(b"Hi\x05");
        drop(p);
        assert_eq!(printed, "Hi");
        assert_eq!(executed, vec![0x05]);
    }

    #[test]
    fn test_parser_utf8_rune() {
        let mut printed = String::new();
        let mut print = |r: char| printed.push(r);
        let handler = Handler {
            print: Some(&mut print),
            ..Default::default()
        };
        let mut p = Parser::with_handler(handler);
        p.parse("界".as_bytes());
        let r = p.rune();
        drop(p);
        assert_eq!(printed, "界");
        assert_eq!(r, '界');
    }

    #[test]
    fn test_cmd_packing() {
        let c = Cmd(command(b'?', 0, b'h'));
        assert_eq!(c.prefix(), b'?');
        assert_eq!(c.intermediate(), 0);
        assert_eq!(c.final_(), b'h');
        let c = Cmd(command(0, b'$', b'p'));
        assert_eq!(c.prefix(), 0);
        assert_eq!(c.intermediate(), b'$');
        assert_eq!(c.final_(), b'p');
    }

    #[test]
    fn test_param_packing() {
        let p = Param(parameter(10, true));
        assert_eq!(p.param(0), 10);
        assert!(p.has_more());
        let p = Param(MISSING_PARAM);
        assert_eq!(p.param(7), 7);
        assert!(!p.has_more());
    }

    #[test]
    fn test_has_prefix_helpers() {
        assert!(has_csi_prefix(b"\x9b"));
        assert!(has_csi_prefix(b"\x1b["));
        assert!(!has_csi_prefix(b"\x1b]"));
        assert!(has_osc_prefix(b"\x1b]"));
        assert!(has_st_prefix(b"\x1b\\"));
        assert!(has_st_prefix(b"\x9c"));
        assert!(has_dcs_prefix(b"\x1bP"));
        assert!(has_sos_prefix(b"\x1bX"));
        assert!(has_pm_prefix(b"\x1b^"));
        assert!(has_apc_prefix(b"\x1b_"));
    }

    #[test]
    fn test_first_grapheme_cluster() {
        let (g, w) =
            first_grapheme_cluster("👍🏽".as_bytes(), crate::method::WidthMethod::GraphemeWidth);
        assert_eq!(g, "👍🏽".as_bytes());
        assert_eq!(w, 2);
    }

    #[test]
    fn test_state_names() {
        assert_eq!(state_name(GROUND_STATE), "GroundState");
        assert_eq!(state_name(UTF8_STATE), "Utf8State");
    }

    /// Ported from upstream `TestDecodeSequence`: sequence boundary table.
    #[test]
    fn test_decode_sequence_table() {
        type Case<'a> = (&'a str, Vec<(&'a [u8], usize, usize)>);
        let cases: &[Case] = &[
            ("single byte", vec![(b"\x1b".as_slice(), 1, 0)]),
            ("single byte 2", vec![(b"\x00".as_slice(), 1, 0)]),
            ("ASCII printable", vec![(b"a".as_slice(), 1, 1)]),
            ("ASCII space", vec![(b" ".as_slice(), 1, 1)]),
            ("ASCII DEL", vec![(b"\x7f".as_slice(), 1, 0)]),
            (
                "double ESC",
                vec![(b"\x1b".as_slice(), 1, 0), (b"\x1b".as_slice(), 1, 0)],
            ),
            (
                "double ST",
                vec![(b"\x1b\\".as_slice(), 2, 0), (b"\x1b\\".as_slice(), 2, 0)],
            ),
            (
                "double ST 8-bit",
                vec![(b"\x9c".as_slice(), 1, 0), (b"\x9c".as_slice(), 1, 0)],
            ),
            (
                "SS3",
                vec![(b"\x1bO".as_slice(), 2, 0), (b"a".as_slice(), 1, 1)],
            ),
            (
                "SS3 8-bit",
                vec![(b"\x8f".as_slice(), 1, 0), (b"a".as_slice(), 1, 1)],
            ),
            (
                "CSI style sequence",
                vec![(b"\x1b[1;2;3m".as_slice(), 8, 0)],
            ),
            ("rune", vec![(b"\xf0\x9f\x91\x8b".as_slice(), 4, 2)]),
            (
                "ESC followed by C0",
                vec![
                    (b"\x1b[".as_slice(), 2, 0),
                    (b"\x00".as_slice(), 1, 0),
                    (b"a".as_slice(), 1, 1),
                ],
            ),
            (
                "ESC sequence with intermediate",
                vec![(b"\x1b Q".as_slice(), 3, 0)],
            ),
        ];
        for (name, expected) in cases {
            let mut state = NORMAL_STATE;
            // Reconstruct input from expected seqs to avoid string escaping issues.
            let input: Vec<u8> = expected
                .iter()
                .flat_map(|(seq, _, _)| seq.iter().copied())
                .collect();
            let mut rest: &[u8] = &input;
            for (seq, n, width) in expected {
                let d = decode_sequence(rest, state, None);
                assert_eq!(d.seq, *seq, "{name}: seq");
                assert_eq!(d.n, *n, "{name}: n for {:?}", std::str::from_utf8(seq).ok());
                assert_eq!(d.width, *width, "{name}: width");
                state = d.state;
                rest = &rest[d.n..];
            }
            assert!(rest.is_empty(), "{name}: leftover {rest:?}");
        }
    }

    /// Ported from upstream `TestDecodeSequence`: DCS with embedded DEL/ST.
    #[test]
    fn test_decode_dcs_del_and_st() {
        // DCS with DEL inside data.
        let input = b"\x1bP1;2+xa\x7fb\x1b\\";
        let mut p = new_parser();
        let d = decode_sequence(input, NORMAL_STATE, Some(&mut p));
        assert_eq!(d.n, input.len());
        assert_eq!(p.data(), b"a\x7fb");
        assert_eq!(p.cmd & 0xff, b'x' as i32);

        // ST 8-bit terminates DCS mid-way.
        let input = b"\x1bP1;2+xa\x9cb\x1b\\";
        let mut p = new_parser();
        let d = decode_sequence(input, NORMAL_STATE, Some(&mut p));
        assert_eq!(d.n, 9);
        assert_eq!(p.data(), b"a");
        // The remainder continues normally.
        let rest = &input[d.n..];
        let d = decode_sequence(rest, NORMAL_STATE, None);
        assert_eq!(d.seq, b"b");
        assert_eq!(d.width, 1);
    }

    /// Ported from upstream: OSC with 8-bit ST, and ESC sequences following OSC.
    #[test]
    fn test_decode_osc_variants() {
        // 8-bit ST terminator.
        let input = b"\x1b]11;ff/00/ff\x9c\x1baa\x8fa";
        let mut p = new_parser();
        let d = decode_sequence(input, NORMAL_STATE, Some(&mut p));
        assert_eq!(d.n, 14);
        assert_eq!(p.cmd, 11);

        // OSC followed by ESC sequence (ESC cancels the string).
        let input = b"\x1b]11;ff/00/ff\x1b[1;2;3m";
        let mut p = new_parser();
        let d = decode_sequence(input, NORMAL_STATE, Some(&mut p));
        assert_eq!(d.n, 13);
        assert_eq!(p.cmd, 11);

        // OSC terminated by bare ESC (cancelled at end).
        let input = b"\x1b]11;ff/00/ff\x1b";
        let mut p = new_parser();
        let d = decode_sequence(input, NORMAL_STATE, Some(&mut p));
        assert_eq!(d.n, 13);
        assert_eq!(p.cmd, 11);

        // Single-param OSC.
        let input = b"\x1b]112\x07";
        let mut p = new_parser();
        let d = decode_sequence(input, NORMAL_STATE, Some(&mut p));
        assert_eq!(d.n, input.len());
        assert_eq!(p.cmd, 112);
        assert_eq!(p.data(), b"112");
    }

    /// 8-bit C1 CSI/DCS/OSC/SOS/PM/APC entries.
    #[test]
    fn test_decode_c1_sequences() {
        let mut p = new_parser();
        let d = decode_sequence(b"\x9b31m", NORMAL_STATE, Some(&mut p));
        assert_eq!(d.n, 4);
        let cmd = Cmd(p.command());
        assert_eq!(cmd.final_(), b'm');

        let mut p = new_parser();
        let d = decode_sequence(b"\x9d2;title\x9c", NORMAL_STATE, Some(&mut p));
        assert_eq!(d.n, 9);
        assert_eq!(p.cmd, 2);

        // CSI with prefix and colon sub-params.
        let mut p = new_parser();
        let d = decode_sequence(b"\x1b[?1:2:3m", NORMAL_STATE, Some(&mut p));
        assert_eq!(d.n, 9);
        let cmd = Cmd(p.command());
        assert_eq!(cmd.prefix(), b'?');
        assert_eq!(cmd.final_(), b'm');
        let (first, has_more, _) = p.params().param(0, 0);
        assert_eq!(first, 1);
        assert!(has_more);
    }

    /// Unterminated and invalid sequences.
    #[test]
    fn test_decode_invalid() {
        // Unterminated CSI.
        let input = b"\x1b[1;2;3";
        let mut p = new_parser();
        let d = decode_sequence(input, NORMAL_STATE, Some(&mut p));
        assert_eq!(d.n, input.len());

        // Invalid escape: ESC followed by a byte outside 0x30..=0x7e.
        let input = b"\x1b\x01";
        let d = decode_sequence(input, NORMAL_STATE, None);
        assert_eq!(d.n, 1);

        // Invalid UTF-8 lead byte.
        let d = decode_sequence(b"\xff", NORMAL_STATE, None);
        assert_eq!(d.n, 0);

        // Unterminated DCS is consumed fully.
        let input = b"\x1bP1;2+xa";
        let mut p = new_parser();
        let d = decode_sequence(input, NORMAL_STATE, Some(&mut p));
        assert_eq!(d.n, input.len());
        assert_eq!(p.data(), b"a");
    }

    /// Command packing per upstream `TestCommand`.
    #[test]
    fn test_command_packing_table() {
        assert_eq!(command(0, 0, b'A'), b'A' as i32);
        assert_eq!(
            command(b'?', 0, b'h'),
            b'h' as i32 | (b'?' as i32) << PREFIX_SHIFT
        );
        assert_eq!(
            command(0, b' ', b'q'),
            b'q' as i32 | (b' ' as i32) << INTERMED_SHIFT
        );
        assert_eq!(
            command(b'>', b'(', b'x'),
            b'x' as i32 | (b'>' as i32) << PREFIX_SHIFT | (b'(' as i32) << INTERMED_SHIFT
        );
        assert_eq!(command(0, 0, 11), 11);
    }

    /// Parameter packing per upstream `TestParameter`.
    #[test]
    fn test_parameter_packing_table() {
        assert_eq!(parameter(1, false), 1);
        assert_eq!(parameter(1, true), 1 | HAS_MORE_FLAG);
        assert_eq!(parameter(-1, false), PARAM_MASK);
        assert_eq!(parameter(-1, true), -1);
    }

    /// Parser pool and reset behavior.
    #[test]
    fn test_parser_pool() {
        let mut p = get_parser();
        assert_eq!(p.params_len, 0);
        put_parser(&mut p);
    }

    /// utf8_byte_len classification.
    #[test]
    fn test_utf8_byte_len() {
        assert_eq!(utf8_byte_len(0b0111_1111), 1);
        assert_eq!(utf8_byte_len(0b1100_0000), 2);
        assert_eq!(utf8_byte_len(0b1110_0000), 3);
        assert_eq!(utf8_byte_len(0b1111_0000), 4);
        assert_eq!(utf8_byte_len(0b1111_1000), -1);
    }

    /// The decode entry points route through the method variants.
    #[test]
    fn test_decode_wc_variant() {
        let d = decode_sequence_wc("😀".as_bytes(), NORMAL_STATE, None);
        assert_eq!(d.width, 2);
        let d = decode_sequence("😀".as_bytes(), NORMAL_STATE, None);
        assert_eq!(d.width, 2);
    }

    /// The execute helper writes bytes to a sink.
    #[test]
    fn test_execute_helper() {
        let mut out: Vec<u8> = Vec::new();
        execute(&mut out, "abc").unwrap();
        assert_eq!(out, b"abc");
    }

    /// Parser data/param sizing, for_each, and reset.
    #[test]
    fn test_parser_configuration() {
        let mut p = new_parser();
        // Unlimited data buffer (size 0).
        p.set_data_size(0);
        assert_eq!(p.data_len, -1);
        // Fixed data buffer.
        p.set_data_size(16);
        assert_eq!(p.data_len, 0);
        // Out-of-range param falls back to the default with ok=false.
        p.params_len = 2;
        p.params[0] = 10;
        assert_eq!(p.param(0, 99), (10, true));
        assert_eq!(p.param(2, 99), (99, false));
        assert_eq!(p.param(5, 99), (99, false));
        // for_each iterates params with has_more flags.
        let mut visited = Vec::new();
        p.params[0] = parameter(1, true);
        p.params[1] = parameter(2, false);
        p.params()
            .for_each(0, |i, v, has_more| visited.push((i, v, has_more)));
        assert_eq!(visited, vec![(0, 1, true), (1, 2, false)]);
        // reset clears to ground state.
        p.state = CSI_PARAM_STATE;
        p.reset();
        assert_eq!(p.state, GROUND_STATE);
        assert_eq!(p.params_len, 0);
        assert_eq!(p.cmd, 0);
        assert_eq!(p.state_name(), "GroundState");
    }

    /// Parser rune/control accessors.
    #[test]
    fn test_parser_rune_and_control() {
        let mut p = new_parser();
        // 4-byte rune packed little-endian into cmd.
        p.cmd = 0xf0_i32 | (0x9f_i32 << 8) | (0x98_i32 << 16) | (0x80_i32 << 24);
        p.params_len = 4;
        assert_eq!(p.rune(), '😀');
        // 1-byte control.
        p.cmd = 0x05;
        assert_eq!(p.control(), 0x05);
        // Invalid rune length.
        p.cmd = 0xff;
        p.params_len = 1;
        assert_eq!(p.rune(), '\u{FFFD}');
    }

    /// parse_string_cmd decodes the leading digits of the collected data.
    #[test]
    fn test_parse_string_cmd_impl() {
        let mut p = new_parser();
        p.set_data_size(16);
        p.data = b"1337;rest".to_vec();
        p.data_len = 9;
        p.cmd = MISSING_COMMAND;
        p.parse_string_cmd();
        assert_eq!(p.cmd, 1337);

        // Non-digit data does not set the command.
        let mut p = new_parser();
        p.set_data_size(16);
        p.data = b"abc".to_vec();
        p.data_len = 3;
        p.cmd = MISSING_COMMAND;
        p.parse_string_cmd();
        assert_eq!(p.cmd, MISSING_COMMAND);
    }

    /// DCS string handling exercises the START/PUT/Dispatch actions.
    #[test]
    fn test_parser_dcs_handler() {
        let mut events = Vec::new();
        let mut dcs = |cmd: Cmd, params: &[i32], data: &[u8]| {
            events.push(format!(
                "dcs:{:?}:{}:{:?}",
                cmd.0,
                params.len(),
                String::from_utf8_lossy(data)
            ));
        };
        let handler = Handler {
            handle_dcs: Some(&mut dcs),
            ..Default::default()
        };
        let mut p = Parser::with_handler(handler);
        p.parse(b"\x1bPq#0data\x1b\\");
        drop(p);
        assert_eq!(events.len(), 1);
        assert!(events[0].starts_with("dcs:"));
        assert!(events[0].contains("data"));
    }

    /// OSC handlers fire on the command and data.
    #[test]
    fn test_parser_osc_handler() {
        let mut events = Vec::new();
        let mut osc = |cmd: i32, data: &[u8]| {
            events.push(format!("osc:{}:{:?}", cmd, String::from_utf8_lossy(data)));
        };
        let handler = Handler {
            handle_osc: Some(&mut osc),
            ..Default::default()
        };
        let mut p = Parser::with_handler(handler);
        p.parse(b"\x1b]2;title\x07");
        drop(p);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], "osc:2:\"2;title\"");
    }

    /// C1 string-state sequences reach the handler.
    #[test]
    fn test_parser_apc_handler() {
        let mut events = Vec::new();
        let mut apc = |data: &[u8]| {
            events.push(format!("apc:{:?}", String::from_utf8_lossy(data)));
        };
        let handler = Handler {
            handle_apc: Some(&mut apc),
            ..Default::default()
        };
        let mut p = Parser::with_handler(handler);
        p.parse(b"\x9fpayload\x9c");
        drop(p);
        assert_eq!(events.len(), 1);
        assert!(events[0].starts_with("apc:"));
    }

    /// OSC with an 8-bit ST terminator.
    #[test]
    fn test_parser_osc_st8() {
        let mut events = Vec::new();
        let mut osc = |cmd: i32, _: &[u8]| events.push(cmd);
        let handler = Handler {
            handle_osc: Some(&mut osc),
            ..Default::default()
        };
        let mut p = Parser::with_handler(handler);
        p.parse(b"\x9d11;ff/00/ff\x9c");
        drop(p);
        assert_eq!(events, vec![11]);
    }

    /// ESC sequence (two-char, e.g. ESC 7) dispatches a command.
    #[test]
    fn test_parser_esc_dispatch() {
        let mut events = Vec::new();
        let mut esc = |cmd: Cmd| events.push(format!("esc:{:?}", cmd.0));
        let handler = Handler {
            handle_esc: Some(&mut esc),
            ..Default::default()
        };
        let mut p = Parser::with_handler(handler);
        p.parse(b"\x1b7");
        drop(p);
        assert_eq!(events, vec!["esc:55"]);
    }

    /// OSC command parsing when data does not start with a digit.
    #[test]
    fn test_parser_osc_non_numeric() {
        let mut events = Vec::new();
        let mut osc = |cmd: i32, _: &[u8]| events.push(cmd);
        let handler = Handler {
            handle_osc: Some(&mut osc),
            ..Default::default()
        };
        let mut p = Parser::with_handler(handler);
        p.parse(b"\x1b]L;abc\x07");
        drop(p);
        // The OSC handler fires even with a missing command (no leading digit).
        assert_eq!(events, vec![MISSING_COMMAND]);
    }
}
