//FileSignature1, FileSignature2 and FileVersion32 in KeePassLib/Serialization/KdbxFile.cs
#[allow(dead_code)]
pub const SIG1: u32 = 0x9AA2_D903;
pub const SIG2: u32 = 0xB54B_FB67;
pub const VERSION_40: u32 = 0x0004_0000;
pub const VERSION_41: u32 = 0x0004_0001;
#[allow(dead_code)]
pub const VD_VER: u16 = 0x0100;
#[allow(dead_code)]
pub const HEADER_BLK_IDX: u64 = ::std::u64::MAX;
#[allow(dead_code)]
pub const SALSA20_IV: &[u8] = &[0xE8, 0x30, 0x09, 0x4B, 0x97, 0x20, 0x5D, 0x2A];
#[allow(dead_code)]
pub const EMPTY: &[u8] = &[];

//Encrypted data is split into blocks of this size before prefixed with its hmac
pub const PAYLOAD_BLOCK_SIZE: u64 = 1048576; // (1MB = 1024 * 1024), 65536  1048576

pub const INTERNAL_VERSION: i32 = 1;
pub const GENERATOR_NAME: &str = "OneKeyPass";
pub const EMPTY_STR: &str = "";

// All Custom Data keys are of pattern OKP_K*. This is used instead of some descriptive
// string to reduce number of bytes taken by the key name entries in db and thus overall size of db
// Any new key should have the next OKP_Kx.
// Do not change the existing key to make sure for backward compatability
pub mod custom_data_key {
    pub const OKP_INTERNAL_VERSION: &str = "OKP_K1";
    pub const OKP_GROUP_AS_CATEGORY: &str = "OKP_K2";
    // Key for the name of the Entry type
    pub const OKP_ENTRY_TYPE: &str = "OKP_K3";
    // The value found corresponding to this custom data key is the serialized Entry type Data
    pub const OKP_ENTRY_TYPE_DATA: &str = "OKP_K4";
    // The value found corresponding to this custom data key is the serialized
    // HashMap of Entry type Data
    pub const OKP_ENTRY_TYPE_MAP_DATA: &str = "OKP_K5";
    // The value associated to this key is the list of serialized Entry type Data
    // that may be used in an entry's history entries
    pub const OKP_ENTRY_TYPE_LIST_DATA: &str = "OKP_K6";
    // The index in place of Entry type Data in the history entry
    pub const OKP_ENTRY_TYPE_DATA_INDEX: &str = "OKP_K7";
}

#[allow(dead_code)]
pub mod entry_type_name {
    pub const LOGIN: &str = "Login";
    pub const WIRELESS_ROUTER: &str = "Wireless Router";
    pub const CREDIT_DEBIT_CARD: &str = "Credit/Debit Card";
    pub const BANK_ACCOUNT: &str = "Bank Account";
    pub const PASSPORT: &str = "Passport";
    pub const IDENTITY: &str = "Identity";
    pub const DRIVER_LICENSE: &str = "Driver License";

    //Medical Record, Membership,
}

// We can generate hex bytes like this using
// let uid = Uuid::new_v4();
// println!("{}",util::as_hex_array_formatted(uid.as_bytes()));
// println!("{}",uid.to_string());

#[allow(dead_code)]
pub mod entry_type_uuid {
    //ffef5f51-7efc-4373-9eb5-382d5b501768
    pub const LOGIN: &[u8] = &[
        0xFF, 0xEF, 0x5F, 0x51, 0x7E, 0xFC, 0x43, 0x73, 0x9E, 0xB5, 0x38, 0x2D, 0x5B, 0x50, 0x17,
        0x68,
    ];

    //9e644c27-d00b-4aca-8355-5078c5a4fb44
    pub const WIRELESS_ROUTER: &[u8] = &[
        0x9E, 0x64, 0x4C, 0x27, 0xD0, 0x0B, 0x4A, 0xCA, 0x83, 0x55, 0x50, 0x78, 0xC5, 0xA4, 0xFB,
        0x44,
    ];

    //c83aa78a-3a8c-45fc-b0e1-08002d166544
    pub const CREDIT_DEBIT_CARD: &[u8] = &[
        0xC8, 0x3A, 0xA7, 0x8A, 0x3A, 0x8C, 0x45, 0xFC, 0xB0, 0xE1, 0x08, 0x00, 0x2D, 0x16, 0x65,
        0x44,
    ];
    //713850b6-9457-45ca-a861-0402db2ca98f
    pub const BANK_ACCOUNT: &[u8] = &[
        0x71, 0x38, 0x50, 0xB6, 0x94, 0x57, 0x45, 0xCA, 0xA8, 0x61, 0x04, 0x02, 0xDB, 0x2C, 0xA9,
        0x8F,
    ];

    //0ba6d80b-8b8d-4ccd-b0dc-840337951cb0
    pub const PASSPORT: &[u8] = &[
        0x0B, 0xA6, 0xD8, 0x0B, 0x8B, 0x8D, 0x4C, 0xCD, 0xB0, 0xDC, 0x84, 0x03, 0x37, 0x95, 0x1C,
        0xB0,
    ];

    //e5aff423-1044-40fe-9565-c0e8dde626c2
    pub const IDENTITY: &[u8] = &[
        0xE5, 0xAF, 0xF4, 0x23, 0x10, 0x44, 0x40, 0xFE, 0x95, 0x65, 0xC0, 0xE8, 0xDD, 0xE6, 0x26,
        0xC2,
    ];

    //90ac9d76-7ea7-4176-b5d0-fabf8a9a0058
    pub const DRIVER_LICENSE: &[u8] = &[
        0x90, 0xAC, 0x9D, 0x76, 0x7E, 0xA7, 0x41, 0x76, 0xB5, 0xD0, 0xFA, 0xBF, 0x8A, 0x9A, 0x00,
        0x58,
    ];
}

#[allow(dead_code)]
pub mod entry_keyvalue_key {
    pub const TITLE: &str = "Title";
    pub const NOTES: &str = "Notes";
    pub const PASSWORD: &str = "Password";
    pub const URL: &str = "URL";
    pub const USER_NAME: &str = "UserName";
    pub const NUMBER: &str = "Number";
}

//#[allow(non_upper_case_globals)]

#[allow(dead_code)]
pub mod xml_element {
    pub const KEEPASS_FILE: &[u8] = b"KeePassFile";
    pub const META: &[u8] = b"Meta";
    pub const ROOT: &[u8] = b"Root";

    //Meta
    pub const GENERATOR: &[u8] = b"Generator";
    pub const DATABASE_NAME: &[u8] = b"DatabaseName";
    pub const DATABASE_DESCRIPTION: &[u8] = b"DatabaseDescription";
    pub const SETTINGS_CHANGED: &[u8] = b"SettingsChanged";

    //pub const :&[u8]  = b"MasterKeyChanged";
    pub const DATABASE_NAME_CHANGED: &[u8] = b"DatabaseNameChanged"; //date time
    pub const RECYCLE_BIN_ENABLED: &[u8] = b"RecycleBinEnabled";
    pub const RECYCLE_BIN_UUID: &[u8] = b"RecycleBinUUID";
    pub const RECYCLE_BIN_CHANGED: &[u8] = b"RecycleBinChanged";
    pub const HISTORY_MAX_ITEMS: &[u8] = b"HistoryMaxItems";
    pub const MAINTENANCE_HISTORY_DAYS: &[u8] = b"MaintenanceHistoryDays";
    pub const HISTORY_MAX_SIZE: &[u8] = b"HistoryMaxSize";
    pub const LAST_SELECTED_GROUP: &[u8] = b"LastSelectedGroup";

    pub const MEMORY_PROTECTION: &[u8] = b"MemoryProtection";
    pub const PROTECT_TITLE: &[u8] = b"ProtectTitle";
    pub const PROTECT_USER_NAME: &[u8] = b"ProtectUserName";
    pub const PROTECT_PASSWORD: &[u8] = b"ProtectPassword";
    pub const PROTECT_URL: &[u8] = b"ProtectURL";
    pub const PROTECT_NOTES: &[u8] = b"ProtectNotes";

    pub const CUSTOM_ICONS: &[u8] = b"CustomIcons";
    pub const ICON: &[u8] = b"Icon";
    pub const DATA: &[u8] = b"Data";

    //Some Common tags

    //Custom Data
    pub const CUSTOM_DATA: &[u8] = b"CustomData";
    pub const ITEM: &[u8] = b"Item";

    //Times
    pub const TIMES: &[u8] = b"Times";
    pub const LAST_MODIFICATION_TIME: &[u8] = b"LastModificationTime";
    pub const CREATION_TIME: &[u8] = b"CreationTime";
    pub const LAST_ACCESS_TIME: &[u8] = b"LastAccessTime";
    pub const EXPIRES: &[u8] = b"Expires";
    pub const EXPIRY_TIME: &[u8] = b"ExpiryTime";
    pub const LOCATION_CHANGED: &[u8] = b"LocationChanged";
    pub const USAGE_COUNT: &[u8] = b"UsageCount";

    //pub const :&[u8]  = b"";
    pub const NOTES: &[u8] = b"Notes";
    pub const TAGS: &[u8] = b"Tags";

    pub const GROUP: &[u8] = b"Group";
    pub const UUID: &[u8] = b"UUID";
    pub const NAME: &[u8] = b"Name";
    pub const ICON_ID: &[u8] = b"IconID";
    pub const LAST_TOP_VISIBLE_ENTRY: &[u8] = b"LastTopVisibleEntry";
    pub const IS_EXPANDED: &[u8] = b"IsExpanded";

    //

    //Entry
    pub const ENTRY: &[u8] = b"Entry";
    pub const BINARY: &[u8] = b"Binary";
    pub const STRING: &[u8] = b"String";
    pub const KEY: &[u8] = b"Key";
    pub const VALUE: &[u8] = b"Value"; //
    pub const HISTORY: &[u8] = b"History";

    //pub const KEEPASS_FILE_TAGS:&[&[u8]] = &[META,ROOT];
}

pub mod key_file_xml_element {
    pub const KEY_FILE: &[u8] = b"KeyFile";
    pub const KEY_FILE_META: &[u8] = b"Meta";
    pub const KEY_FILE_VERSION: &[u8] = b"Version"; 
    pub const KEY_FILE_KEY: &[u8] = b"Key";
    pub const KEY_FILE_DATA: &[u8] = b"Data";
    pub const KEY_FILE_DATA_HASH: &[u8] = b"Hash";
}


#[allow(dead_code)]
pub mod uuid {
    pub const ARGON2_KDF: &[u8] = &[
        0xEF, 0x63, 0x6D, 0xDF, 0x8C, 0x29, 0x44, 0x4B, 0x91, 0xF7, 0xA9, 0xA4, 0x03, 0xE3, 0x0A,
        0x0C,
    ];
    pub const AES_KDF: &[u8] = &[
        0xC9, 0xD9, 0xF3, 0x9A, 0x62, 0x8A, 0x44, 0x60, 0xBF, 0x74, 0x0D, 0x08, 0xC1, 0x8A, 0x4F,
        0xEA,
    ];

    pub const CHACHA20: &[u8] = &[
        0xD6, 0x03, 0x8A, 0x2B, 0x8B, 0x6F, 0x4C, 0xB5, 0xA5, 0x24, 0x33, 0x9A, 0x31, 0xDB, 0xB5,
        0x9A,
    ];
    pub const AES256: &[u8] = &[
        0x31, 0xC1, 0xF2, 0xE6, 0xBF, 0x71, 0x43, 0x50, 0xBE, 0x58, 0x05, 0x21, 0x6A, 0xFC, 0x5A,
        0xFF,
    ];
}
//Types used in VariantDictionary VdType
#[allow(dead_code)]
pub mod vd_type {
    pub const NONE: u8 = 0x00;
    pub const UINT32: u8 = 0x04;
    pub const UINT64: u8 = 0x05;
    pub const BOOL: u8 = 0x08;
    pub const INT32: u8 = 0x0C;
    pub const INT64: u8 = 0x0D;
    pub const STRING: u8 = 0x18;
    pub const BYTEARRAY: u8 = 0x42;
}

#[allow(dead_code)]
pub mod vd_param {
    pub const UUID: &str = "$UUID";

    pub mod argon2 {
        pub const SALT: &str = "S";
        pub const PARALLELISM: &str = "P";
        pub const MEMORY: &str = "M";
        pub const ITERATIONS: &str = "I";
        pub const VERSION: &str = "V";
        pub const SECRETKEY: &str = "K";
        pub const ASSOCDATA: &str = "A";

        pub const DEFAULT_ITERATIONS: &[u8] = &[0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        pub const DEFAULT_PARALLELISM: &[u8] = &[0x02, 0x00, 0x00, 0x00];
        pub const DEFAULT_MEMORY: &[u8] = &[0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00];
    }

    pub mod aes {
        pub const ROUNDS: &str = "R";
        pub const SEED: &str = "S";

        pub const DEFAULT_ROUNDS: &[u8] = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    }
}
// All ids from enum KdbxHeaderFieldID of KeePassLib/Serialization/KdbxFile.cs and Only KDBX 4 ids are considered
#[allow(dead_code)]
pub mod header_type {
    pub const END_OF_HEADER: u8 = 0;
    pub const COMMENT: u8 = 1;
    pub const CIPHER_ID: u8 = 2;
    pub const COMPRESSION_FLAGS: u8 = 3;
    pub const MASTER_SEED: u8 = 4;
    pub const ENCRYPTION_IV: u8 = 7;
    pub const KDF_PARAMETERS: u8 = 11;
    pub const PUBLIC_CUSTOM_DATA: u8 = 12;
}

#[allow(dead_code)]
pub mod inner_header_type {
    pub const END_OF_HEADER: u8 = 0x00;
    pub const STREAM_ID: u8 = 0x01;
    pub const STREAM_KEY: u8 = 0x02;
    pub const BINARY: u8 = 0x03;

    pub const BINARY_PROTECTED: u8 = 0x01;
    pub const BINARY_PLAIN: u8 = 0x00;

    pub const SALSA20_STREAM: u32 = 2; //LE Bytes (2 0 0 0)
    pub const CHACHA20_STREAM: u32 = 3; //LE Bytes (3 0 0 0)
}
