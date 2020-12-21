const ALL_KEYS: &'static [&'static str] =
	&["KEY_RESERVED",
		"KEY_ESC",
		"KEY_1",
		"KEY_2",
		"KEY_3",
		"KEY_4",
		"KEY_5",
		"KEY_6",
		"KEY_7",
		"KEY_8",
		"KEY_9",
		"KEY_10",
		"KEY_MINUS",
		"KEY_EQUAL",
		"KEY_BACKSPACE",
		"KEY_TAB",
		"KEY_Q",
		"KEY_W",
		"KEY_E",
		"KEY_R",
		"KEY_T",
		"KEY_Y",
		"KEY_U",
		"KEY_I",
		"KEY_O",
		"KEY_P",
		"KEY_LEFTBRACE",
		"KEY_RIGHTBRACE",
		"KEY_ENTER",
		"KEY_LEFTCTRL",
		"KEY_A",
		"KEY_S",
		"KEY_D",
		"KEY_F",
		"KEY_G",
		"KEY_H",
		"KEY_J",
		"KEY_K",
		"KEY_L",
		"KEY_SEMICOLON",
		"KEY_APOSTROPHE",
		"KEY_GRAVE",
		"KEY_LEFTSHIFT",
		"KEY_BACKSLASH",
		"KEY_Z",
		"KEY_X",
		"KEY_C",
		"KEY_V",
		"KEY_B",
		"KEY_N",
		"KEY_M",
		"KEY_COMMA",
		"KEY_DOT",
		"KEY_SLASH",
		"KEY_RIGHTSHIFT",
		"KEY_KPASTERISK",
		"KEY_LEFTALT",
		"KEY_SPACE",
		"KEY_CAPSLOCK",
		"KEY_F1",
		"KEY_F2",
		"KEY_F3",
		"KEY_F4",
		"KEY_F5",
		"KEY_F6",
		"KEY_F7",
		"KEY_F8",
		"KEY_F9",
		"KEY_F10",
		"KEY_NUMLOCK",
		"KEY_SCROLLLOCK",
		"KEY_KP7",
		"KEY_KP8",
		"KEY_KP9",
		"KEY_KPMINUS",
		"KEY_KP4",
		"KEY_KP5",
		"KEY_KP6",
		"KEY_KPPLUS",
		"KEY_KP1",
		"KEY_KP2",
		"KEY_KP3",
		"KEY_KP0",
		"KEY_KPDOT",
		"KEY_ZENKAKUHANKAKU",
		"KEY_102ND",
		"KEY_F11",
		"KEY_F12",
		"KEY_RO",
		"KEY_KATAKANA",
		"KEY_HIRAGANA",
		"KEY_HENKAN",
		"KEY_KATAKANAHIRAGANA",
		"KEY_MUHENKAN",
		"KEY_KPJPCOMMA",
		"KEY_KPENTER",
		"KEY_RIGHTCTRL",
		"KEY_KPSLASH",
		"KEY_SYSRQ",
		"KEY_RIGHTALT",
		"KEY_LINEFEED",
		"KEY_HOME",
		"KEY_UP",
		"KEY_PAGEUP",
		"KEY_LEFT",
		"KEY_RIGHT",
		"KEY_END",
		"KEY_DOWN",
		"KEY_PAGEDOWN",
		"KEY_INSERT",
		"KEY_DELETE",
		"KEY_MACRO",
		"KEY_MUTE",
		"KEY_VOLUMEDOWN",
		"KEY_VOLUMEUP",
		"KEY_POWER",
		"KEY_KPEQUAL",
		"KEY_KPPLUSMINUS",
		"KEY_PAUSE",
		"KEY_SCALE",
		"KEY_KPCOMMA",
		"KEY_HANGEUL",
		"KEY_HANGUEL",
		"KEY_HANJA",
		"KEY_YEN",
		"KEY_LEFTMETA",
		"KEY_RIGHTMETA",
		"KEY_COMPOSE",
		"KEY_STOP",
		"KEY_AGAIN",
		"KEY_PROPS",
		"KEY_UNDO",
		"KEY_FRONT",
		"KEY_COPY",
		"KEY_OPEN",
		"KEY_PASTE",
		"KEY_FIND",
		"KEY_CUT",
		"KEY_HELP",
		"KEY_MENU",
		"KEY_CALC",
		"KEY_SETUP",
		"KEY_SLEEP",
		"KEY_WAKEUP",
		"KEY_FILE",
		"KEY_SENDFILE",
		"KEY_DELETEFILE",
		"KEY_XFER",
		"KEY_PROG1",
		"KEY_PROG2",
		"KEY_WWW",
		"KEY_MSDOS",
		"KEY_COFFEE",
		"KEY_SCREENLOCK",
		"KEY_ROTATE_DISPLAY",
		"KEY_DIRECTION",
		"KEY_CYCLEWINDOWS",
		"KEY_MAIL",
		"KEY_BOOKMARKS",
		"KEY_COMPUTER",
		"KEY_BACK",
		"KEY_FORWARD",
		"KEY_CLOSECD",
		"KEY_EJECTCD",
		"KEY_EJECTCLOSECD",
		"KEY_NEXTSONG",
		"KEY_PLAYPAUSE",
		"KEY_PREVIOUSSONG",
		"KEY_STOPCD",
		"KEY_RECORD",
		"KEY_REWIND",
		"KEY_PHONE",
		"KEY_ISO",
		"KEY_CONFIG",
		"KEY_HOMEPAGE",
		"KEY_REFRESH",
		"KEY_EXIT",
		"KEY_MOVE",
		"KEY_EDIT",
		"KEY_SCROLLUP",
		"KEY_SCROLLDOWN",
		"KEY_KPLEFTPAREN",
		"KEY_KPRIGHTPAREN",
		"KEY_NEW",
		"KEY_REDO",
		"KEY_F13",
		"KEY_F14",
		"KEY_F15",
		"KEY_F16",
		"KEY_F17",
		"KEY_F18",
		"KEY_F19",
		"KEY_F20",
		"KEY_F21",
		"KEY_F22",
		"KEY_F23",
		"KEY_F24",
		"KEY_PLAYCD",
		"KEY_PAUSECD",
		"KEY_PROG3",
		"KEY_PROG4",
		"KEY_DASHBOARD",
		"KEY_SUSPEND",
		"KEY_CLOSE",
		"KEY_PLAY",
		"KEY_FASTFORWARD",
		"KEY_BASSBOOST",
		"KEY_PRINT",
		"KEY_HP",
		"KEY_CAMERA",
		"KEY_SOUND",
		"KEY_QUESTION",
		"KEY_EMAIL",
		"KEY_CHAT",
		"KEY_SEARCH",
		"KEY_CONNECT",
		"KEY_FINANCE",
		"KEY_SPORT",
		"KEY_SHOP",
		"KEY_ALTERASE",
		"KEY_CANCEL",
		"KEY_BRIGHTNESSDOWN",
		"KEY_BRIGHTNESSUP",
		"KEY_MEDIA",
		"KEY_SWITCHVIDEOMODE",
		"KEY_KBDILLUMTOGGLE",
		"KEY_KBDILLUMDOWN",
		"KEY_KBDILLUMUP",
		"KEY_SEND",
		"KEY_REPLY",
		"KEY_FORWARDMAIL",
		"KEY_SAVE",
		"KEY_DOCUMENTS",
		"KEY_BATTERY",
		"KEY_BLUETOOTH",
		"KEY_WLAN",
		"KEY_UWB",
		"KEY_UNKNOWN",
		"KEY_VIDEO_NEXT",
		"KEY_VIDEO_PREV",
		"KEY_BRIGHTNESS_CYCLE",
		"KEY_BRIGHTNESS_AUTO",
		"KEY_BRIGHTNESS_ZERO",
		"KEY_DISPLAY_OFF",
		"KEY_WWAN",
		"KEY_WIMAX",
		"KEY_RFKILL",
		"KEY_MICMUTE",
		"KEY_OK",
		"KEY_SELECT",
		"KEY_GOTO",
		"KEY_CLEAR",
		"KEY_POWER2",
		"KEY_OPTION",
		"KEY_INFO",
		"KEY_TIME",
		"KEY_VENDOR",
		"KEY_ARCHIVE",
		"KEY_PROGRAM",
		"KEY_CHANNEL",
		"KEY_FAVORITES",
		"KEY_EPG",
		"KEY_PVR",
		"KEY_MHP",
		"KEY_LANGUAGE",
		"KEY_TITLE",
		"KEY_SUBTITLE",
		"KEY_ANGLE",
		"KEY_ZOOM",
		"KEY_MODE",
		"KEY_KEYBOARD",
		"KEY_SCREEN",
		"KEY_PC",
		"KEY_TV",
		"KEY_TV2",
		"KEY_VCR",
		"KEY_VCR2",
		"KEY_SAT",
		"KEY_SAT2",
		"KEY_CD",
		"KEY_TAPE",
		"KEY_RADIO",
		"KEY_TUNER",
		"KEY_PLAYER",
		"KEY_TEXT",
		"KEY_DVD",
		"KEY_AUX",
		"KEY_MP3",
		"KEY_AUDIO",
		"KEY_VIDEO",
		"KEY_DIRECTORY",
		"KEY_LIST",
		"KEY_MEMO",
		"KEY_CALENDAR",
		"KEY_RED",
		"KEY_GREEN",
		"KEY_YELLOW",
		"KEY_BLUE",
		"KEY_CHANNELUP",
		"KEY_CHANNELDOWN",
		"KEY_FIRST",
		"KEY_LAST",
		"KEY_AB",
		"KEY_NEXT",
		"KEY_RESTART",
		"KEY_SLOW",
		"KEY_SHUFFLE",
		"KEY_BREAK",
		"KEY_PREVIOUS",
		"KEY_DIGITS",
		"KEY_TEEN",
		"KEY_TWEN",
		"KEY_VIDEOPHONE",
		"KEY_GAMES",
		"KEY_ZOOMIN",
		"KEY_ZOOMOUT",
		"KEY_ZOOMRESET",
		"KEY_WORDPROCESSOR",
		"KEY_EDITOR",
		"KEY_SPREADSHEET",
		"KEY_GRAPHICSEDITOR",
		"KEY_PRESENTATION",
		"KEY_DATABASE",
		"KEY_NEWS",
		"KEY_VOICEMAIL",
		"KEY_ADDRESSBOOK",
		"KEY_MESSENGER",
		"KEY_DISPLAYTOGGLE",
		"KEY_BRIGHTNESS_TOGGLE",
		"KEY_SPELLCHECK",
		"KEY_LOGOFF",
		"KEY_DOLLAR",
		"KEY_EURO",
		"KEY_FRAMEBACK",
		"KEY_FRAMEFORWARD",
		"KEY_CONTEXT_MENU",
		"KEY_MEDIA_REPEAT",
		"KEY_10CHANNELSUP",
		"KEY_10CHANNELSDOWN",
		"KEY_IMAGES",
		"KEY_DEL_EOL",
		"KEY_DEL_EOS",
		"KEY_INS_LINE",
		"KEY_DEL_LINE",
		"KEY_FN",
		"KEY_FN_ESC",
		"KEY_FN_F1",
		"KEY_FN_F2",
		"KEY_FN_F3",
		"KEY_FN_F4",
		"KEY_FN_F5",
		"KEY_FN_F6",
		"KEY_FN_F7",
		"KEY_FN_F8",
		"KEY_FN_F9",
		"KEY_FN_F10",
		"KEY_FN_F11",
		"KEY_FN_F12",
		"KEY_FN_1",
		"KEY_FN_2",
		"KEY_FN_D",
		"KEY_FN_E",
		"KEY_FN_F",
		"KEY_FN_S",
		"KEY_FN_B",
		"KEY_BRL_DOT1",
		"KEY_BRL_DOT2",
		"KEY_BRL_DOT3",
		"KEY_BRL_DOT4",
		"KEY_BRL_DOT5",
		"KEY_BRL_DOT6",
		"KEY_BRL_DOT7",
		"KEY_BRL_DOT8",
		"KEY_BRL_DOT9",
		"KEY_BRL_DOT10",
		"KEY_NUMERIC_0",
		"KEY_NUMERIC_1",
		"KEY_NUMERIC_2",
		"KEY_NUMERIC_3",
		"KEY_NUMERIC_4",
		"KEY_NUMERIC_5",
		"KEY_NUMERIC_6",
		"KEY_NUMERIC_7",
		"KEY_NUMERIC_8",
		"KEY_NUMERIC_9",
		"KEY_NUMERIC_STAR",
		"KEY_NUMERIC_POUND",
		"KEY_NUMERIC_A",
		"KEY_NUMERIC_B",
		"KEY_NUMERIC_C",
		"KEY_NUMERIC_D",
		"KEY_CAMERA_FOCUS",
		"KEY_WPS_BUTTON",
		"KEY_TOUCHPAD_TOGGLE",
		"KEY_TOUCHPAD_ON",
		"KEY_TOUCHPAD_OFF",
		"KEY_CAMERA_ZOOMIN",
		"KEY_CAMERA_ZOOMOUT",
		"KEY_CAMERA_UP",
		"KEY_CAMERA_DOWN",
		"KEY_CAMERA_LEFT",
		"KEY_CAMERA_RIGHT",
		"KEY_ATTENDANT_ON",
		"KEY_ATTENDANT_OFF",
		"KEY_ATTENDANT_TOGGLE",
		"KEY_LIGHTS_TOGGLE",
		"KEY_ALS_TOGGLE",
		"KEY_BUTTONCONFIG",
		"KEY_TASKMANAGER",
		"KEY_JOURNAL",
		"KEY_CONTROLPANEL",
		"KEY_APPSELECT",
		"KEY_SCREENSAVER",
		"KEY_VOICECOMMAND",
		"KEY_BRIGHTNESS_MIN",
		"KEY_BRIGHTNESS_MAX",
		"KEY_KBDINPUTASSIST_PREV",
		"KEY_KBDINPUTASSIST_NEXT",
		"KEY_KBDINPUTASSIST_PREVGROUP",
		"KEY_KBDINPUTASSIST_NEXTGROUP",
		"KEY_KBDINPUTASSIST_ACCEPT",
		"KEY_KBDINPUTASSIST_CANCEL",
		"KEY_MIN_INTERESTING",
		"KEY_MAX",
		"KEY_CNT"];

pub fn code_to_name(x: i32) -> &'static str {
	for k in ALL_KEYS {
		if name_to_code(k) == x {
			return *k;
		}
	}
	panic!("Keycode not recognized {}", x);
}


pub fn name_to_code(s: &str) -> i32 {
	match s {
		_ if ALL_KEYS[0].eq(s) => 0,
		_ if ALL_KEYS[1].eq(s) => 1,
		_ if ALL_KEYS[2].eq(s) => 2,
		_ if ALL_KEYS[3].eq(s) => 3,
		_ if ALL_KEYS[4].eq(s) => 4,
		_ if ALL_KEYS[5].eq(s) => 5,
		_ if ALL_KEYS[6].eq(s) => 6,
		_ if ALL_KEYS[7].eq(s) => 7,
		_ if ALL_KEYS[8].eq(s) => 8,
		_ if ALL_KEYS[9].eq(s) => 9,
		_ if ALL_KEYS[10].eq(s) => 10,
		_ if ALL_KEYS[11].eq(s) => 11,
		_ if ALL_KEYS[12].eq(s) => 12,
		_ if ALL_KEYS[13].eq(s) => 13,
		_ if ALL_KEYS[14].eq(s) => 14,
		_ if ALL_KEYS[15].eq(s) => 15,
		_ if ALL_KEYS[16].eq(s) => 16,
		_ if ALL_KEYS[17].eq(s) => 17,
		_ if ALL_KEYS[18].eq(s) => 18,
		_ if ALL_KEYS[19].eq(s) => 19,
		_ if ALL_KEYS[20].eq(s) => 20,
		_ if ALL_KEYS[21].eq(s) => 21,
		_ if ALL_KEYS[22].eq(s) => 22,
		_ if ALL_KEYS[23].eq(s) => 23,
		_ if ALL_KEYS[24].eq(s) => 24,
		_ if ALL_KEYS[25].eq(s) => 25,
		_ if ALL_KEYS[26].eq(s) => 26,
		_ if ALL_KEYS[27].eq(s) => 27,
		_ if ALL_KEYS[28].eq(s) => 28,
		_ if ALL_KEYS[29].eq(s) => 29,
		_ if ALL_KEYS[30].eq(s) => 30,
		_ if ALL_KEYS[31].eq(s) => 31,
		_ if ALL_KEYS[32].eq(s) => 32,
		_ if ALL_KEYS[33].eq(s) => 33,
		_ if ALL_KEYS[34].eq(s) => 34,
		_ if ALL_KEYS[35].eq(s) => 35,
		_ if ALL_KEYS[36].eq(s) => 36,
		_ if ALL_KEYS[37].eq(s) => 37,
		_ if ALL_KEYS[38].eq(s) => 38,
		_ if ALL_KEYS[39].eq(s) => 39,
		_ if ALL_KEYS[40].eq(s) => 40,
		_ if ALL_KEYS[41].eq(s) => 41,
		_ if ALL_KEYS[42].eq(s) => 42,
		_ if ALL_KEYS[43].eq(s) => 43,
		_ if ALL_KEYS[44].eq(s) => 44,
		_ if ALL_KEYS[45].eq(s) => 45,
		_ if ALL_KEYS[46].eq(s) => 46,
		_ if ALL_KEYS[47].eq(s) => 47,
		_ if ALL_KEYS[48].eq(s) => 48,
		_ if ALL_KEYS[49].eq(s) => 49,
		_ if ALL_KEYS[50].eq(s) => 50,
		_ if ALL_KEYS[51].eq(s) => 51,
		_ if ALL_KEYS[52].eq(s) => 52,
		_ if ALL_KEYS[53].eq(s) => 53,
		_ if ALL_KEYS[54].eq(s) => 54,
		_ if ALL_KEYS[55].eq(s) => 55,
		_ if ALL_KEYS[56].eq(s) => 56,
		_ if ALL_KEYS[57].eq(s) => 57,
		_ if ALL_KEYS[58].eq(s) => 58,
		_ if ALL_KEYS[59].eq(s) => 59,
		_ if ALL_KEYS[60].eq(s) => 60,
		_ if ALL_KEYS[61].eq(s) => 61,
		_ if ALL_KEYS[62].eq(s) => 62,
		_ if ALL_KEYS[63].eq(s) => 63,
		_ if ALL_KEYS[64].eq(s) => 64,
		_ if ALL_KEYS[65].eq(s) => 65,
		_ if ALL_KEYS[66].eq(s) => 66,
		_ if ALL_KEYS[67].eq(s) => 67,
		_ if ALL_KEYS[68].eq(s) => 68,
		_ if ALL_KEYS[69].eq(s) => 69,
		_ if ALL_KEYS[70].eq(s) => 70,
		_ if ALL_KEYS[71].eq(s) => 71,
		_ if ALL_KEYS[72].eq(s) => 72,
		_ if ALL_KEYS[73].eq(s) => 73,
		_ if ALL_KEYS[74].eq(s) => 74,
		_ if ALL_KEYS[75].eq(s) => 75,
		_ if ALL_KEYS[76].eq(s) => 76,
		_ if ALL_KEYS[77].eq(s) => 77,
		_ if ALL_KEYS[78].eq(s) => 78,
		_ if ALL_KEYS[79].eq(s) => 79,
		_ if ALL_KEYS[80].eq(s) => 80,
		_ if ALL_KEYS[81].eq(s) => 81,
		_ if ALL_KEYS[82].eq(s) => 82,
		_ if ALL_KEYS[83].eq(s) => 83,
		_ if ALL_KEYS[84].eq(s) => 85,
		_ if ALL_KEYS[85].eq(s) => 86,
		_ if ALL_KEYS[86].eq(s) => 87,
		_ if ALL_KEYS[87].eq(s) => 88,
		_ if ALL_KEYS[88].eq(s) => 89,
		_ if ALL_KEYS[89].eq(s) => 90,
		_ if ALL_KEYS[90].eq(s) => 91,
		_ if ALL_KEYS[91].eq(s) => 92,
		_ if ALL_KEYS[92].eq(s) => 93,
		_ if ALL_KEYS[93].eq(s) => 94,
		_ if ALL_KEYS[94].eq(s) => 95,
		_ if ALL_KEYS[95].eq(s) => 96,
		_ if ALL_KEYS[96].eq(s) => 97,
		_ if ALL_KEYS[97].eq(s) => 98,
		_ if ALL_KEYS[98].eq(s) => 99,
		_ if ALL_KEYS[99].eq(s) => 100,
		_ if ALL_KEYS[100].eq(s) => 101,
		_ if ALL_KEYS[101].eq(s) => 102,
		_ if ALL_KEYS[102].eq(s) => 103,
		_ if ALL_KEYS[103].eq(s) => 104,
		_ if ALL_KEYS[104].eq(s) => 105,
		_ if ALL_KEYS[105].eq(s) => 106,
		_ if ALL_KEYS[106].eq(s) => 107,
		_ if ALL_KEYS[107].eq(s) => 108,
		_ if ALL_KEYS[108].eq(s) => 109,
		_ if ALL_KEYS[109].eq(s) => 110,
		_ if ALL_KEYS[110].eq(s) => 111,
		_ if ALL_KEYS[111].eq(s) => 112,
		_ if ALL_KEYS[112].eq(s) => 113,
		_ if ALL_KEYS[113].eq(s) => 114,
		_ if ALL_KEYS[114].eq(s) => 115,
		_ if ALL_KEYS[115].eq(s) => 116,
		_ if ALL_KEYS[116].eq(s) => 117,
		_ if ALL_KEYS[117].eq(s) => 118,
		_ if ALL_KEYS[118].eq(s) => 119,
		_ if ALL_KEYS[119].eq(s) => 120,
		_ if ALL_KEYS[120].eq(s) => 121,
		_ if ALL_KEYS[121].eq(s) => 122,
		_ if ALL_KEYS[122].eq(s) => 122,
		_ if ALL_KEYS[123].eq(s) => 123,
		_ if ALL_KEYS[124].eq(s) => 124,
		_ if ALL_KEYS[125].eq(s) => 125,
		_ if ALL_KEYS[126].eq(s) => 126,
		_ if ALL_KEYS[127].eq(s) => 127,
		_ if ALL_KEYS[128].eq(s) => 128,
		_ if ALL_KEYS[129].eq(s) => 129,
		_ if ALL_KEYS[130].eq(s) => 130,
		_ if ALL_KEYS[131].eq(s) => 131,
		_ if ALL_KEYS[132].eq(s) => 132,
		_ if ALL_KEYS[133].eq(s) => 133,
		_ if ALL_KEYS[134].eq(s) => 134,
		_ if ALL_KEYS[135].eq(s) => 135,
		_ if ALL_KEYS[136].eq(s) => 136,
		_ if ALL_KEYS[137].eq(s) => 137,
		_ if ALL_KEYS[138].eq(s) => 138,
		_ if ALL_KEYS[139].eq(s) => 139,
		_ if ALL_KEYS[140].eq(s) => 140,
		_ if ALL_KEYS[141].eq(s) => 141,
		_ if ALL_KEYS[142].eq(s) => 142,
		_ if ALL_KEYS[143].eq(s) => 143,
		_ if ALL_KEYS[144].eq(s) => 144,
		_ if ALL_KEYS[145].eq(s) => 145,
		_ if ALL_KEYS[146].eq(s) => 146,
		_ if ALL_KEYS[147].eq(s) => 147,
		_ if ALL_KEYS[148].eq(s) => 148,
		_ if ALL_KEYS[149].eq(s) => 149,
		_ if ALL_KEYS[150].eq(s) => 150,
		_ if ALL_KEYS[151].eq(s) => 151,
		_ if ALL_KEYS[152].eq(s) => 152,
		_ if ALL_KEYS[153].eq(s) => 152,
		_ if ALL_KEYS[154].eq(s) => 153,
		_ if ALL_KEYS[155].eq(s) => 153,
		_ if ALL_KEYS[156].eq(s) => 154,
		_ if ALL_KEYS[157].eq(s) => 155,
		_ if ALL_KEYS[158].eq(s) => 156,
		_ if ALL_KEYS[159].eq(s) => 157,
		_ if ALL_KEYS[160].eq(s) => 158,
		_ if ALL_KEYS[161].eq(s) => 159,
		_ if ALL_KEYS[162].eq(s) => 160,
		_ if ALL_KEYS[163].eq(s) => 161,
		_ if ALL_KEYS[164].eq(s) => 162,
		_ if ALL_KEYS[165].eq(s) => 163,
		_ if ALL_KEYS[166].eq(s) => 164,
		_ if ALL_KEYS[167].eq(s) => 165,
		_ if ALL_KEYS[168].eq(s) => 166,
		_ if ALL_KEYS[169].eq(s) => 167,
		_ if ALL_KEYS[170].eq(s) => 168,
		_ if ALL_KEYS[171].eq(s) => 169,
		_ if ALL_KEYS[172].eq(s) => 170,
		_ if ALL_KEYS[173].eq(s) => 171,
		_ if ALL_KEYS[174].eq(s) => 172,
		_ if ALL_KEYS[175].eq(s) => 173,
		_ if ALL_KEYS[176].eq(s) => 174,
		_ if ALL_KEYS[177].eq(s) => 175,
		_ if ALL_KEYS[178].eq(s) => 176,
		_ if ALL_KEYS[179].eq(s) => 177,
		_ if ALL_KEYS[180].eq(s) => 178,
		_ if ALL_KEYS[181].eq(s) => 179,
		_ if ALL_KEYS[182].eq(s) => 180,
		_ if ALL_KEYS[183].eq(s) => 181,
		_ if ALL_KEYS[184].eq(s) => 182,
		_ if ALL_KEYS[185].eq(s) => 183,
		_ if ALL_KEYS[186].eq(s) => 184,
		_ if ALL_KEYS[187].eq(s) => 185,
		_ if ALL_KEYS[188].eq(s) => 186,
		_ if ALL_KEYS[189].eq(s) => 187,
		_ if ALL_KEYS[190].eq(s) => 188,
		_ if ALL_KEYS[191].eq(s) => 189,
		_ if ALL_KEYS[192].eq(s) => 190,
		_ if ALL_KEYS[193].eq(s) => 191,
		_ if ALL_KEYS[194].eq(s) => 192,
		_ if ALL_KEYS[195].eq(s) => 193,
		_ if ALL_KEYS[196].eq(s) => 194,
		_ if ALL_KEYS[197].eq(s) => 200,
		_ if ALL_KEYS[198].eq(s) => 201,
		_ if ALL_KEYS[199].eq(s) => 202,
		_ if ALL_KEYS[200].eq(s) => 203,
		_ if ALL_KEYS[201].eq(s) => 204,
		_ if ALL_KEYS[202].eq(s) => 205,
		_ if ALL_KEYS[203].eq(s) => 206,
		_ if ALL_KEYS[204].eq(s) => 207,
		_ if ALL_KEYS[205].eq(s) => 208,
		_ if ALL_KEYS[206].eq(s) => 209,
		_ if ALL_KEYS[207].eq(s) => 210,
		_ if ALL_KEYS[208].eq(s) => 211,
		_ if ALL_KEYS[209].eq(s) => 212,
		_ if ALL_KEYS[210].eq(s) => 213,
		_ if ALL_KEYS[211].eq(s) => 214,
		_ if ALL_KEYS[212].eq(s) => 215,
		_ if ALL_KEYS[213].eq(s) => 216,
		_ if ALL_KEYS[214].eq(s) => 217,
		_ if ALL_KEYS[215].eq(s) => 218,
		_ if ALL_KEYS[216].eq(s) => 219,
		_ if ALL_KEYS[217].eq(s) => 220,
		_ if ALL_KEYS[218].eq(s) => 221,
		_ if ALL_KEYS[219].eq(s) => 222,
		_ if ALL_KEYS[220].eq(s) => 223,
		_ if ALL_KEYS[221].eq(s) => 224,
		_ if ALL_KEYS[222].eq(s) => 225,
		_ if ALL_KEYS[223].eq(s) => 226,
		_ if ALL_KEYS[224].eq(s) => 227,
		_ if ALL_KEYS[225].eq(s) => 228,
		_ if ALL_KEYS[226].eq(s) => 229,
		_ if ALL_KEYS[227].eq(s) => 230,
		_ if ALL_KEYS[228].eq(s) => 231,
		_ if ALL_KEYS[229].eq(s) => 232,
		_ if ALL_KEYS[230].eq(s) => 233,
		_ if ALL_KEYS[231].eq(s) => 234,
		_ if ALL_KEYS[232].eq(s) => 235,
		_ if ALL_KEYS[233].eq(s) => 236,
		_ if ALL_KEYS[234].eq(s) => 237,
		_ if ALL_KEYS[235].eq(s) => 238,
		_ if ALL_KEYS[236].eq(s) => 239,
		_ if ALL_KEYS[237].eq(s) => 240,
		_ if ALL_KEYS[238].eq(s) => 241,
		_ if ALL_KEYS[239].eq(s) => 242,
		_ if ALL_KEYS[240].eq(s) => 243,
		_ => -1
	}
}