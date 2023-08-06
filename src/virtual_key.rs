macro_rules! virtual_key_enum {
    (
        $(
            $(#[$attr:meta])*
            $variant:ident => $og_ident:ident
        )*
    ) => {
        /// https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum VirtualKey {
            $(
                $(#[$attr])*
                #[doc = ""]
                #[doc = "`"]
                #[doc = stringify!($og_ident)]
                #[doc = "`"]
                // #[doc(alias = stringify!($og_ident))]
                $variant,
            )*
        }

        impl VirtualKey {
            pub fn code(&self) -> windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY {
                match self {
                    $(
                        VirtualKey::$variant => windows::Win32::UI::Input::KeyboardAndMouse::$og_ident,
                    )*
                }
            }
        }
    };
}

virtual_key_enum! {
    /// Numeric 0 key
    /// (Not to be confused with [`VirtualKey::NumPad0`])
    #[doc(alias = "Number0")]
    Num0 => VK_0
    /// Numeric 1 key
    /// (Not to be confused with [`VirtualKey::NumPad1`])
    #[doc(alias = "Number1")]
    Num1 => VK_1
    /// Numeric 2 key
    /// (Not to be confused with [`VirtualKey::NumPad2`])
    #[doc(alias = "Number2")]
    Num2 => VK_2
    /// Numeric 3 key
    /// (Not to be confused with [`VirtualKey::NumPad3`])
    #[doc(alias = "Number3")]
    Num3 => VK_3
    /// Numeric 4 key
    /// (Not to be confused with [`VirtualKey::NumPad4`])
    #[doc(alias = "Number4")]
    Num4 => VK_4
    /// Numeric 5 key
    /// (Not to be confused with [`VirtualKey::NumPad5`])
    #[doc(alias = "Number5")]
    Num5 => VK_5
    /// Numeric 6 key
    /// (Not to be confused with [`VirtualKey::NumPad6`])
    #[doc(alias = "Number6")]
    Num6 => VK_6
    /// Numeric 7 key
    /// (Not to be confused with [`VirtualKey::NumPad7`])
    #[doc(alias = "Number7")]
    Num7 => VK_7
    /// Numeric 8 key
    /// (Not to be confused with [`VirtualKey::NumPad8`])
    #[doc(alias = "Number8")]
    Num8 => VK_8
    /// Numeric 9 key
    /// (Not to be confused with [`VirtualKey::NumPad9`])
    #[doc(alias = "Number9")]
    Num9 => VK_9
    /// A key
    #[doc(alias = "LetterA")]
    A => VK_A
    /// B key
    #[doc(alias = "LetterB")]
    B => VK_B
    /// C key
    #[doc(alias = "LetterC")]
    C => VK_C
    /// D key
    #[doc(alias = "LetterD")]
    D => VK_D
    /// E key
    #[doc(alias = "LetterE")]
    E => VK_E
    /// F key
    #[doc(alias = "LetterF")]
    F => VK_F
    /// G key
    #[doc(alias = "LetterG")]
    G => VK_G
    /// H key
    #[doc(alias = "LetterH")]
    H => VK_H
    /// I key
    #[doc(alias = "LetterI")]
    I => VK_I
    /// J key
    #[doc(alias = "LetterJ")]
    J => VK_J
    /// K key
    #[doc(alias = "LetterK")]
    K => VK_K
    /// L key
    #[doc(alias = "LetterL")]
    L => VK_L
    /// M key
    #[doc(alias = "LetterM")]
    M => VK_M
    /// N key
    #[doc(alias = "LetterN")]
    N => VK_N
    /// O key
    #[doc(alias = "LetterO")]
    O => VK_O
    /// P key
    #[doc(alias = "LetterP")]
    P => VK_P
    /// Q key
    #[doc(alias = "LetterQ")]
    Q => VK_Q
    /// R key
    #[doc(alias = "LetterR")]
    R => VK_R
    /// S key
    #[doc(alias = "LetterS")]
    S => VK_S
    /// T key
    #[doc(alias = "LetterT")]
    T => VK_T
    /// U key
    #[doc(alias = "LetterU")]
    U => VK_U
    /// V key
    #[doc(alias = "LetterV")]
    V => VK_V
    /// W key
    #[doc(alias = "LetterW")]
    W => VK_W
    /// X key
    #[doc(alias = "LetterX")]
    X => VK_X
    /// Y key
    #[doc(alias = "LetterY")]
    Y => VK_Y
    /// Z key
    #[doc(alias = "LetterZ")]
    Z => VK_Z
    AbntC1 => VK_ABNT_C1
    AbntC2 => VK_ABNT_C2
    DbeAlphanumeric => VK_DBE_ALPHANUMERIC
    DbeCodeInput => VK_DBE_CODEINPUT
    DbeDbcsChar => VK_DBE_DBCSCHAR
    DbeDetermineString => VK_DBE_DETERMINESTRING
    DbeEnterDlgConversionMode => VK_DBE_ENTERDLGCONVERSIONMODE
    DbeEnterImeConfigMode => VK_DBE_ENTERIMECONFIGMODE
    DbeEnterWordRegisterMode => VK_DBE_ENTERWORDREGISTERMODE
    DbeFlushString => VK_DBE_FLUSHSTRING
    DbeHiragana => VK_DBE_HIRAGANA
    DbeKatakana => VK_DBE_KATAKANA
    DbeNoCodeInput => VK_DBE_NOCODEINPUT
    DbeNoRoman => VK_DBE_NOROMAN
    DbeRoman => VK_DBE_ROMAN
    DbeSbcsChar => VK_DBE_SBCSCHAR
    // removed because what this do?: _none_ => VK__none_
    /// Left mouse button
    #[doc(alias = "LeftMouseButton")]
    LButton => VK_LBUTTON
    /// Right mouse button
    #[doc(alias = "RightMouseButton")]
    RButton => VK_RBUTTON
    /// Control-break processing
    Cancel => VK_CANCEL
    /// Middle mouse button (three-button mouse)
    #[doc(alias = "MiddleMouseButton")]
    MButton => VK_MBUTTON
    /// X1 mouse button
    #[doc(alias = "ForwardMouseButton")]
    XButton1 => VK_XBUTTON1
    /// X2 mouse button
    #[doc(alias = "BackMouseButton")]
    #[doc(alias = "BackwardMouseButton")]
    XButton2 => VK_XBUTTON2
    /// BACKSPACE key
    #[doc(alias = "Back")]
    Backspace => VK_BACK
    /// TAB key
    /// (Tabulator)
    #[doc(alias = "Tabulator")]
    Tab => VK_TAB
    /// CLEAR key
    Clear => VK_CLEAR
    /// ENTER key
    #[doc(alias = "Return")]
    Enter => VK_RETURN
    /// SHIFT key
    Shift => VK_SHIFT
    /// CTRL key
    #[doc(alias = "Ctrl")]
    Control => VK_CONTROL
    /// ALT key
    #[doc(alias = "Menu")]
    Alt => VK_MENU
    /// PAUSE key
    Pause => VK_PAUSE
    /// CAPS LOCK key
    #[doc(alias = "Capital")]
    CapsLock => VK_CAPITAL
    /// IME Kana mode
    #[doc(alias = "IMEKana")]
    Kana => VK_KANA
    // /// IME Hanguel mode (maintained for compatibility; use VK_HANGUL)
    // removed because deprecated: Hangeul => VK_HANGEUL
    /// IME Hangul mode
    #[doc(alias = "IMEHangul")]
    #[doc(alias = "Hangeul")]
    #[doc(alias = "IMEHangeul")]
    Hangul => VK_HANGUL
    /// IME On
    #[doc(alias = "IMEOn")]
    ImeOn => VK_IME_ON
    /// IME Junja mode
    #[doc(alias = "IMEJunja")]
    Junja => VK_JUNJA
    /// IME final mode
    #[doc(alias = "IMEFinal")]
    Final => VK_FINAL
    /// IME Hanja mode
    #[doc(alias = "IMEHanja")]
    Hanja => VK_HANJA
    /// IME Kanji mode
    #[doc(alias = "IMEKanji")]
    Kanji => VK_KANJI
    /// IME Off
    #[doc(alias = "IMEOff")]
    ImeOff => VK_IME_OFF
    /// ESC key
    #[doc(alias = "Esc")]
    Escape => VK_ESCAPE
    /// IME convert
    #[doc(alias = "IMEConvert")]
    Convert => VK_CONVERT
    /// IME nonconvert
    #[doc(alias = "IMENonConvert")]
    NonConvert => VK_NONCONVERT
    /// IME accept
    #[doc(alias = "IMEAccept")]
    Accept => VK_ACCEPT
    /// IME mode change request
    #[doc(alias = "IMEModeChange")]
    #[doc(alias = "IMEModeChangeRequest")]
    ModeChange => VK_MODECHANGE
    /// SPACEBAR
    #[doc(alias = "SpaceBar")]
    Space => VK_SPACE
    /// PAGE UP key
    #[doc(alias = "Prior")]
    PageUp => VK_PRIOR
    /// PAGE DOWN key
    #[doc(alias = "Next")]
    PageDown => VK_NEXT
    /// END key
    End => VK_END
    /// HOME key
    Home => VK_HOME
    /// LEFT ARROW key
    LeftArrow => VK_LEFT
    /// UP ARROW key
    UpArrow => VK_UP
    /// RIGHT ARROW key
    RightArrow => VK_RIGHT
    /// DOWN ARROW key
    DownArrow => VK_DOWN
    /// SELECT key
    Select => VK_SELECT
    /// PRINT key
    Print => VK_PRINT
    /// EXECUTE key
    Execute => VK_EXECUTE
    /// PRINT SCREEN key
    #[doc(alias = "SnapShot")]
    #[doc(alias = "PrtSc")]
    PrintScreen => VK_SNAPSHOT
    /// INS key
    #[doc(alias = "Ins")]
    Insert => VK_INSERT
    /// DEL key
    #[doc(alias = "Del")]
    Delete => VK_DELETE
    /// HELP key
    Help => VK_HELP
    /// Left Windows key (Natural keyboard)
    #[doc(alias = "LeftWin")]
    #[doc(alias = "LeftWindow")]
    LWin => VK_LWIN
    /// Right Windows key (Natural keyboard)
    #[doc(alias = "RightWin")]
    #[doc(alias = "RightWindow")]
    RWin => VK_RWIN
    /// Applications key (Natural keyboard)
    Apps => VK_APPS
    /// Computer Sleep key
    Sleep => VK_SLEEP
    /// Numeric keypad 0 key
    /// (Not to be confused with [`VirtualKey::Num0`])
    #[doc(alias = "Keypad0")]
    NumPad0 => VK_NUMPAD0
    /// Numeric keypad 1 key
    /// (Not to be confused with [`VirtualKey::Num1`])
    #[doc(alias = "Keypad1")]
    NumPad1 => VK_NUMPAD1
    /// Numeric keypad 2 key
    /// (Not to be confused with [`VirtualKey::Num2`])
    #[doc(alias = "Keypad2")]
    NumPad2 => VK_NUMPAD2
    /// Numeric keypad 3 key
    /// (Not to be confused with [`VirtualKey::Num3`])
    #[doc(alias = "Keypad3")]
    NumPad3 => VK_NUMPAD3
    /// Numeric keypad 4 key
    /// (Not to be confused with [`VirtualKey::Num4`])
    #[doc(alias = "Keypad4")]
    NumPad4 => VK_NUMPAD4
    /// Numeric keypad 5 key
    /// (Not to be confused with [`VirtualKey::Num5`])
    #[doc(alias = "Keypad5")]
    NumPad5 => VK_NUMPAD5
    /// Numeric keypad 6 key
    /// (Not to be confused with [`VirtualKey::Num6`])
    #[doc(alias = "Keypad6")]
    NumPad6 => VK_NUMPAD6
    /// Numeric keypad 7 key
    /// (Not to be confused with [`VirtualKey::Num7`])
    #[doc(alias = "Keypad7")]
    NumPad7 => VK_NUMPAD7
    /// Numeric keypad 8 key
    /// (Not to be confused with [`VirtualKey::Num8`])
    #[doc(alias = "Keypad8")]
    NumPad8 => VK_NUMPAD8
    /// Numeric keypad 9 key
    /// (Not to be confused with [`VirtualKey::Num9`])
    #[doc(alias = "Keypad9")]
    NumPad9 => VK_NUMPAD9
    /// Multiply key
    Multiply => VK_MULTIPLY
    /// Add key
    Add => VK_ADD
    /// Separator key
    Separator => VK_SEPARATOR
    /// Subtract key
    Subtract => VK_SUBTRACT
    /// Decimal key
    Decimal => VK_DECIMAL
    /// Divide key
    Divide => VK_DIVIDE
    /// F1 key
    F1 => VK_F1
    /// F2 key
    F2 => VK_F2
    /// F3 key
    F3 => VK_F3
    /// F4 key
    F4 => VK_F4
    /// F5 key
    F5 => VK_F5
    /// F6 key
    F6 => VK_F6
    /// F7 key
    F7 => VK_F7
    /// F8 key
    F8 => VK_F8
    /// F9 key
    F9 => VK_F9
    /// F10 key
    F10 => VK_F10
    /// F11 key
    F11 => VK_F11
    /// F12 key
    F12 => VK_F12
    /// F13 key
    F13 => VK_F13
    /// F14 key
    F14 => VK_F14
    /// F15 key
    F15 => VK_F15
    /// F16 key
    F16 => VK_F16
    /// F17 key
    F17 => VK_F17
    /// F18 key
    F18 => VK_F18
    /// F19 key
    F19 => VK_F19
    /// F20 key
    F20 => VK_F20
    /// F21 key
    F21 => VK_F21
    /// F22 key
    F22 => VK_F22
    /// F23 key
    F23 => VK_F23
    /// F24 key
    F24 => VK_F24
    NavigationView => VK_NAVIGATION_VIEW
    NavigationMenu => VK_NAVIGATION_MENU
    NavigationUP => VK_NAVIGATION_UP
    NavigationDown => VK_NAVIGATION_DOWN
    NavigationLeft => VK_NAVIGATION_LEFT
    NavigationRight => VK_NAVIGATION_RIGHT
    NavigationAccept => VK_NAVIGATION_ACCEPT
    NavigationCancel => VK_NAVIGATION_CANCEL
    /// NUM LOCK key
    NumLock => VK_NUMLOCK
    /// SCROLL LOCK key
    #[doc(alias = "Scroll")]
    ScrollLock => VK_SCROLL
    OemNecEqual => VK_OEM_NEC_EQUAL
    OemFjJisho => VK_OEM_FJ_JISHO
    OemFjMasshou => VK_OEM_FJ_MASSHOU
    OemFjTouroku => VK_OEM_FJ_TOUROKU
    OemFjLoya => VK_OEM_FJ_LOYA
    OemFjRoya => VK_OEM_FJ_ROYA
    /// Left SHIFT key
    #[doc(alias = "LeftShift")]
    LShift => VK_LSHIFT
    /// Right SHIFT key
    #[doc(alias = "RightShift")]
    RShift => VK_RSHIFT
    /// Left CONTROL key
    #[doc(alias = "LeftControl")]
    LControl => VK_LCONTROL
    /// Right CONTROL key
    #[doc(alias = "RightControl")]
    RControl => VK_RCONTROL
    /// Left ALT key
    #[doc(alias = "LeftMenu")]
    #[doc(alias = "LeftAlt")]
    LAlt => VK_LMENU
    /// Right ALT key
    #[doc(alias = "RightMenu")]
    #[doc(alias = "RightAlt")]
    RAlt => VK_RMENU
    /// Browser Back key
    BrowserBack => VK_BROWSER_BACK
    /// Browser Forward key
    BrowserForward => VK_BROWSER_FORWARD
    /// Browser Refresh key
    BrowserRefresh => VK_BROWSER_REFRESH
    /// Browser Stop key
    BrowserStop => VK_BROWSER_STOP
    /// Browser Search key
    BrowserSearch => VK_BROWSER_SEARCH
    /// Browser Favorites key
    BrowserFavorites => VK_BROWSER_FAVORITES
    /// Browser Start and Home key
    BrowserHome => VK_BROWSER_HOME
    /// Volume Mute key
    VolumeMute => VK_VOLUME_MUTE
    /// Volume Down key
    VolumeDown => VK_VOLUME_DOWN
    /// Volume Up key
    VolumeUp => VK_VOLUME_UP
    /// Next Track key
    MediaNextTrack => VK_MEDIA_NEXT_TRACK
    /// Previous Track key
    MediaPrevTrack => VK_MEDIA_PREV_TRACK
    /// Stop Media key
    MediaStop => VK_MEDIA_STOP
    /// Play/Pause Media key
    MediaPlayPause => VK_MEDIA_PLAY_PAUSE
    /// Start Mail key
    LaunchMail => VK_LAUNCH_MAIL
    /// Select Media key
    LaunchMediaSelect => VK_LAUNCH_MEDIA_SELECT
    /// Start Application 1 key
    LaunchApp1 => VK_LAUNCH_APP1
    /// Start Application 2 key
    LaunchApp2 => VK_LAUNCH_APP2
    /// For any country/region, the `;:` key
    #[doc(alias("SemiColon", "Colon", ";:", ";", ":"))]
    Oem1 => VK_OEM_1
    /// For any country/region, the `+` key
    #[doc(alias("Plus", "+"))]
    OemPlus => VK_OEM_PLUS
    /// For any country/region, the `,` key
    #[doc(alias("Comma", ","))]
    OemComma => VK_OEM_COMMA
    /// For any country/region, the `-` key
    #[doc(alias("Dash", "Minus", "-"))]
    OemMinus => VK_OEM_MINUS
    /// For any country/region, the `.` key
    #[doc(alias("Period", "."))]
    OemPeriod => VK_OEM_PERIOD
    /// Used for miscellaneous characters; it can vary by keyboard.
    /// For the US standard keyboard, the `/?` key
    #[doc(alias(
        "ForwardSlash", "Slash",
        "QuestionMark", "Question",
        "/?", "/", "?"
    ))]
    Oem2 => VK_OEM_2
    /// Used for miscellaneous characters; it can vary by keyboard.
    /// For the US standard keyboard, the `\`~` key
    #[doc(alias(
        "BackTick", "Tick",
        "Tilde", "Squiggle", "Squiggly", "Swiggle", "Twiddle",
        "`~", "`", "~"
    ))]
    Oem3 => VK_OEM_3
    GamepadA => VK_GAMEPAD_A
    GamepadB => VK_GAMEPAD_B
    GamepadX => VK_GAMEPAD_X
    GamepadY => VK_GAMEPAD_Y
    GamepadRightShoulder => VK_GAMEPAD_RIGHT_SHOULDER
    GamepadLeftShoulder => VK_GAMEPAD_LEFT_SHOULDER
    GamepadLeftTrigger => VK_GAMEPAD_LEFT_TRIGGER
    GamepadRightTrigger => VK_GAMEPAD_RIGHT_TRIGGER
    GamepadDPadUp => VK_GAMEPAD_DPAD_UP
    GamepadDPadDown => VK_GAMEPAD_DPAD_DOWN
    GamepadDPadLeft => VK_GAMEPAD_DPAD_LEFT
    GamepadDPadRight => VK_GAMEPAD_DPAD_RIGHT
    GamepadMenu => VK_GAMEPAD_MENU
    GamepadView => VK_GAMEPAD_VIEW
    GamepadLeftThumbStickButton => VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON
    GamepadRightThumbStickButton => VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON
    GamepadLeftThumbStickUp => VK_GAMEPAD_LEFT_THUMBSTICK_UP
    GamepadLeftThumbStickDown => VK_GAMEPAD_LEFT_THUMBSTICK_DOWN
    GamepadLeftThumbStickRight => VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT
    GamepadLeftThumbStickLeft => VK_GAMEPAD_LEFT_THUMBSTICK_LEFT
    GamepadRightThumbStickUp => VK_GAMEPAD_RIGHT_THUMBSTICK_UP
    GamepadRightThumbStickDown => VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN
    GamepadRightThumbStickRight => VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT
    GamepadRightThumbStickLeft => VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT
    /// Used for miscellaneous characters; it can vary by keyboard.
    /// For the US standard keyboard, the `[{` key
    #[doc(alias(
        "Parenthesis", "Parentheses","Bracket", "Brace", "Chevron",
        "SquareBracket",  "OpeningSquareBracket",
        "CurlyBracket",  "OpeningCurlyBracket",
        "[{", "[", "{"
    ))]
    Oem4 => VK_OEM_4
    /// Used for miscellaneous characters; it can vary by keyboard.
    /// For the US standard keyboard, the `\|` key
    #[doc(alias(
        "Slash", "BackSlash",
        "Pipe", "VerticalPipe", "VerticalBar",
        "\\|", "\\", "|"
    ))]
    Oem5 => VK_OEM_5
    /// Used for miscellaneous characters; it can vary by keyboard.
    /// For the US standard keyboard, the `]}` key
    #[doc(alias(
        "Parenthesis", "Parentheses","Bracket", "Brace", "Chevron",
        "SquareBracket",  "ClosingSquareBracket",
        "CurlyBracket",  "ClosingCurlyBracket",
        "]}", "]", "}"
    ))]
    Oem6 => VK_OEM_6
    /// Used for miscellaneous characters; it can vary by keyboard.
    /// For the US standard keyboard, the `'"` key
    #[doc(alias("Quote", "SingleQuote", "DoubleQuote"))]
    // Quotes in doc alises is not allowed ¯\_(ツ)_/¯
    Oem7 => VK_OEM_7
    /// Used for miscellaneous characters; it can vary by keyboard.
    Oem8 => VK_OEM_8
    OemAx => VK_OEM_AX
    /// The `<>` keys on the US standard keyboard,
    /// or the `\|` key on the non-US 102-key keyboard
    #[doc(alias(
        "Parenthesis", "Parentheses","Bracket", "Brace", "Chevron",
        "OpeningAngleBracket", "ClosingAngleBracket",  "AngleBracket",
        "<>", "<", ">", "\\|", "\\", "|"
    ))]
    Oem102 => VK_OEM_102
    IcoHelp => VK_ICO_HELP
    Ico00 => VK_ICO_00
    /// IME PROCESS key
    #[doc(alias = "IMEProcessKey")]
    Process => VK_PROCESSKEY
    IcoClear => VK_ICO_CLEAR
    // /// Used to pass Unicode characters as if they were keystrokes. The VK_PACKET key is the low word of a 32-bit Virtual Key value used for non-keyboard input methods. For more information, see Remark in <a href="/en-us/windows/win32/api/winuser/ns-winuser-keybdinput" data-linktype="absolute-path">KEYBDINPUT</a>, <a href="/en-us/windows/win32/api/winuser/nf-winuser-sendinput" data-linktype="absolute-path">SendInput</a>, <a href="wm-keydown" data-linktype="relative-path">WM_KEYDOWN</a>, and <a href="wm-keyup" data-linktype="relative-path">WM_KEYUP</a>
    // removed because it's a bit idk, not for normal user Packet => VK_PACKET
    OemReset => VK_OEM_RESET
    OemJump => VK_OEM_JUMP
    OemPa1 => VK_OEM_PA1
    OemPa2 => VK_OEM_PA2
    OemPa3 => VK_OEM_PA3
    OemWsctrl => VK_OEM_WSCTRL
    OemCusel => VK_OEM_CUSEL
    OemAttn => VK_OEM_ATTN
    OemFinish => VK_OEM_FINISH
    OemCopy => VK_OEM_COPY
    OemAuto => VK_OEM_AUTO
    OemEnlw => VK_OEM_ENLW
    OemBacktab => VK_OEM_BACKTAB
    /// Attn key
    /// (Attention)
    #[doc(alias = "Attention")]
    Attn => VK_ATTN
    /// CrSel key
    /// (Cursor Select)
    #[doc(alias = "CursorSelect")]
    CrSel => VK_CRSEL
    /// ExSel key
    /// (Extended Selection)
    #[doc(alias = "ExtendedSelection")]
    ExSel => VK_EXSEL
    /// Erase EOF key
    /// (Erase to end of field)
    #[doc(alias = "EraseEOF")]
    #[doc(alias = "EraseToEndOfField")]
    ErEof => VK_EREOF
    /// Play key
    Play => VK_PLAY
    /// Zoom key
    Zoom => VK_ZOOM
    /// Reserved
    NoName => VK_NONAME
    /// PA1 key
    /// (Program Action Key)
    Pa1 => VK_PA1
    /// Clear key
    OemClear => VK_OEM_CLEAR
}
