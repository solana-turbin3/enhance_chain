// struct IndexBucket {
//     mmap: Mmap,                 // Memory-mapped file for fast access
//     bitvec: BitVec,             // Marks which entries are occupied
//     entries: Vec<IndexEntry>,   // Stores account index data
// }

// #[repr(C)]
// struct IndexEntry {
//     pubkey_hash: u64,           // Hash of the account pubkey
//     state: IndexState,          // Indicates if the slot has one or multiple accounts
//     data_offset: u64,           // Points to account data in DataBucket
// }

// #[derive(Clone, Copy)]
// enum IndexState {
//     Free,              // Not occupied
//     ZeroSlots,         // Allocated but empty //Reserved for an account, but no data exists yet.
//     SingleElement,     // Stores one slot for the pubkey
//     MultipleSlots,     // Stores multiple slots in DataBucket
// }


//////////////


// struct DataBucket {
//     mmap: Mmap,                // Memory-mapped file storing account references
//     slots: Vec<SlotEntry>,     // Stores account data references
// }

// #[repr(C)]
// struct SlotEntry {
//     slot: u64,                 // The slot where this account exists
//     file_id: u32,              // Account file identifier
//     offset: u64,               // Offset inside the account file
// }


//////////////


// struct Bucket {
//     index_bucket: IndexBucket,    // Stores (pubkey → (file_id, offset))
//     data_buckets: Vec<DataBucket> // Stores multiple versions of an account (if needed)
// }


//////////////

// Bucket
// ├── IndexBucket (RAM + mmap)
// │   ├── IndexEntry { pubkey_hash, state, data_offset }
// │   ├── IndexEntry { pubkey_hash, state, data_offset }
// │   ├── ...
// ├── DataBuckets (if MultipleSlots)
// │   ├── DataBucket
// │   │   ├── SlotEntry { slot, file_id, offset }
// │   │   ├── SlotEntry { slot, file_id, offset }
// │   │   ├── ...
// │   ├── DataBucket
// │   │   ├── SlotEntry { slot, file_id, offset }


//////////////

// IndexBucket:
// +------------+------------------+-----------+
// | PubkeyHash | IndexState       | DataOffset|
// +------------+------------------+-----------+
// | 0xABCD1234 | SingleElement    | 0x400     |
// +------------+------------------+-----------+

// Account Data (from DataOffset = 0x400)
// +-------+------------+------------+
// | Slot  | File_ID    | Offset     |
// +-------+------------+------------+
// | 1005  | 42        | 0x800      |
// +-------+------------+------------+

// -> Latest slot = 1005


//////////////

// IndexBucket:
// +------------+------------------+-----------+
// | PubkeyHash | IndexState       | DataOffset|
// +------------+------------------+-----------+
// | 0xABCD1234 | MultipleSlots    | 0x1000    |
// +------------+------------------+-----------+

// DataBucket (DataOffset = 0x1000)
// +-------+------------+------------+
// | Slot  | File_ID    | Offset     |
// +-------+------------+------------+
// | 1001  | 40        | 0x600      |
// | 1002  | 41        | 0x700      |
// | 1005  | 42        | 0x800      | <-- Latest slot
// +-------+------------+------------+

// -> Latest slot = 1005
