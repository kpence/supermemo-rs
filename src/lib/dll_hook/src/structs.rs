#![feature(libc)]

use libc::size_t;
use std::ptr;

#[derive(Default, Debug, Copy, Clone)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct ItemOptimizationData {
    pub ElementNo: u32,
    pub Today: u32,
    pub TheGrade: u8,
    pub TheFirstGrade: u8,
    pub LastRepetition: u32,
    pub OldInterval: u32,
    pub UsedInterval: u32,
    pub VirtualUsedInterval: u32,
    pub OI16: u32,
    pub NI16: u32,
    pub OI17: f64,
    pub NI17: f64,
    pub NewInterval: u32,
    pub Repetitions: u16,
    pub NewRepetitions: u16,
    pub Lapses: u16,
    pub NewLapses: u16,
    pub RequestedFI: u8,
    pub Ordinal: f64,
    pub AFactor: f64,
    pub NewAFactor: f64,
    pub UFactor: f64,
    pub NewUFactor: f64,
    pub OldRF: f64,
    pub NewRF: f64,
    pub OldOF: f64,
    pub NewOF: f64,
    pub Cases: u32,
    pub EstimatedFI: f64,
    pub ExpectedFI: f64,
    pub NormalizedGrade: f64,
    pub NGMin: f64,
    pub NGMax: f64,
    pub RepetitionsCategory: f64,
    pub RepetitionsHistory: u32,
    pub ExpR: f64,
    pub UsualR: f64,
    pub PredictedR: f64,
    pub Postpones: u32,
    pub _ItemDifficulty: f64,
}

#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct OptimizationRecord {
    pub OFM: [u16; 400],
    pub RFM: [u16; 400],
    pub Cases: [u16; 400],
    pub DFM: [u16; 19],
    pub DFMCases: [u16; 19],
    pub Ret: [u8; 8000],
    pub RetCases: [u8; 8000],
    pub FirstGradeGraph: [u16; 60],
    pub FirstGradeCases: [u16; 20],
    pub FIGradeGraph: [u16; 90],
    pub FIGradeCases: [u16; 30],
}

impl Default for OptimizationRecord {
    fn default() -> OptimizationRecord {
        OptimizationRecord {
            OFM: [0; 400],
            RFM: [0; 400],
            Cases: [0; 400],
            DFM: [0; 19],
            DFMCases: [0; 19],
            Ret: [0; 8000],
            RetCases: [0; 8000],
            FirstGradeGraph: [0; 60],
            FirstGradeCases: [0; 20],
            FIGradeGraph: [0; 90],
            FIGradeCases: [0; 30],
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct FIVsPriorityRec {
    pub Count: u16,
    pub FI_sum: [u8; 6],
}

#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct DataRecord {
    pub Total: u32,
    pub MemorizedCount: u32,
    pub AllocatedFileSpaceSlots: u32,
    pub Version: u8,
    pub ItemBurden: [u8; 6],
    pub FirstDay: u16,
    pub OldestLastDay: u16,
    pub DefaultFI: u8,
    pub MeasuredFI: [u8; 6],
    pub MeasuredFICases: u16,
    pub FutureReps: [u8; 6],
    pub AILR: [u8; 6],
    pub AFI: [u8; 6],
    pub AIR: [u8; 6],
    pub AL: [u8; 6],
    pub AII: [u8; 6],
    pub AIAF: [u8; 6],
    pub ATim: [u8; 6],
    pub AG: [u8; 6],
    pub AFactorDistribution: [u16; 20],
    pub IntervalDistribution: [u16; 13],
    pub LapsesDistribution: [u8; 30],
    pub UserAddress: [u8; 256],
    pub UserOrderDate: [u8; 13],
    pub _Unused1: [u8; 8],
    pub TotalRepetitionTime: [u8; 8],
    pub TotalRepetitionsCounter: u32,
    pub ATCount: u16,
    pub _Unused2: [u8; 7],
    pub UserEMail: [u8; 50],
    pub _Unused3: [u8; 12],
    pub UserPassword: [u8; 11],
    pub RegisteredPassword: [u8; 11],
    pub CreditCounter: u32,
    pub ItemTotal: u32,
    pub ItemMemorized: u32,
    pub TopicBurden: [u8; 6],
    pub TopicFutureReps: [u8; 6],
    pub AP: [u8; 6],
    pub TAP: [u8; 6],
    pub ATR: [u8; 6],
    pub TAL: [u8; 6],
    pub ATI: [u8; 6],
    pub ATAF: [u8; 6],
    pub ATLR: [u8; 6],
    pub TopicAFactorDistribution: [u16; 20],
    pub TopicIntervalDistribution: [u16; 13],
    pub TopicRepetitionsDistribution: [u16; 20],
    pub TopicTotalRepetitionTime: u64,
    pub TopicTotalRepetitionsCounter: u32,
    pub TopicATCount: u16,
    pub FI_vs_Priority: [FIVsPriorityRec; 20],
}

impl Default for DataRecord {
    fn default() -> DataRecord {
        DataRecord {
            Total: 0,
            MemorizedCount: 0,
            AllocatedFileSpaceSlots: 0,
            Version: 0,
            ItemBurden: [0; 6],
            FirstDay: 0,
            OldestLastDay: 0,
            DefaultFI: 0,
            MeasuredFI: [0; 6],
            MeasuredFICases: 0,
            FutureReps: [0; 6],
            AILR: [0; 6],
            AFI: [0; 6],
            AIR: [0; 6],
            AL: [0; 6],
            AII: [0; 6],
            AIAF: [0; 6],
            ATim: [0; 6],
            AG: [0; 6],
            AFactorDistribution: [0; 20],
            IntervalDistribution: [0; 13],
            LapsesDistribution: [0; 30],
            UserAddress: [0; 256],
            UserOrderDate: [0; 13],
            _Unused1: [0; 8],
            TotalRepetitionTime: [0; 8],
            TotalRepetitionsCounter: 0,
            ATCount: 0,
            _Unused2: [0; 7],
            UserEMail: [0; 50],
            _Unused3: [0; 12],
            UserPassword: [0; 11],
            RegisteredPassword: [0; 11],
            CreditCounter: 0,
            ItemTotal: 0,
            ItemMemorized: 0,
            TopicBurden: [0; 6],
            TopicFutureReps: [0; 6],
            AP: [0; 6],
            TAP: [0; 6],
            ATR: [0; 6],
            TAL: [0; 6],
            ATI: [0; 6],
            ATAF: [0; 6],
            ATLR: [0; 6],
            TopicAFactorDistribution: [0; 20],
            TopicIntervalDistribution: [0; 13],
            TopicRepetitionsDistribution: [0; 20],
            TopicTotalRepetitionTime: 0,
            TopicTotalRepetitionsCounter: 0,
            TopicATCount: 0,
            FI_vs_Priority: [FIVsPriorityRec{Count: 0, FI_sum: [0; 6]}; 20],
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct Database {
    pub _unknown0: u32,
    pub hINF: u32,
    pub _unknown1: u32,
    pub Filespace: u32,
    pub Opened: bool,
    pub DuringEntry: bool,
    pub ResettingCollection: bool,
    pub DrillRandomized: bool,
    pub MidnightShiftWarningShown: bool,
    pub DatabaseName: u32,
    pub FilterFile: u32,
    pub SystemTranslation: u32,
    pub Phonetics: u32,
    pub TextPath: u32,
    pub _unknown2: u32,
    pub _unknown3: u32,
    pub _unknown4: u32,
    pub _unknown5: u32,
    pub _unknown6: u32,
    pub _unknown7: u32,
    pub _unknown8: u8,
    pub _unknown9: u8,
    pub _unknown10: u8,
    pub _unknown11: u8,
    pub _unknown12: u8,
    pub _unknown13: u8,
    pub _unknown14: u8,
    pub _unknown15: u8,
    pub _unknown16: u8,
    pub _unknown17: u8,
    pub _unknown18: u8,
    pub _unknown19: u8,
    pub _unknown20: u8,
    pub _unknown21: u8,
    pub _unknown22: u8,
    pub _unknown23: u8,
    pub _unknown24: u8,
    pub _unknown25: u8,
    pub _unknown26: u8,
    pub _unknown27: u8,
    pub _unknown28: u8,
    pub _unknown29: u8,
    pub _unknown30: u8,
    pub _unknown31: u8,
    pub SourceTextPath: u32,
    pub PhoneticFont: u32,
    pub _unknown32: u8,
    pub _unknown33: u8,
    pub _unknown34: u8,
    pub _unknown35: u8,
    pub _unknown36: u8,
    pub _unknown37: u8,
    pub _unknown38: u8,
    pub _unknown39: u8,
    pub _unknown40: u8,
    pub Pending: *mut Queue,
    pub Drill: *mut Queue,
    pub PQueue: *mut Queue,
    pub RepDates: *mut Queue,
    pub Outstanding: *mut Queue,
    pub OutstandingItems: *mut Queue,
    pub OutstandingTopics: *mut Queue,
    pub DataRecord: DataRecord,
    pub OptimizationRecord: *mut OptimizationRecord,
    pub _unknown41: u8,
    pub _unknown42: u8,
    pub _unknown43: u8,
    pub _unknown44: u8,
    pub _unknown45: u8,
    pub _unknown46: u8,
    pub _unknown47: u8,
    pub _unknown48: u8,
    pub _unknown49: u8,
    pub _unknown50: u8,
    pub _unknown51: u8,
    pub _unknown52: u8,
    pub _unknown53: u8,
    pub _unknown54: u8,
    pub _unknown55: u8,
    pub _unknown56: u8,
    pub _unknown57: u8,
    pub _unknown58: u8,
    pub _unknown59: u8,
    pub _unknown60: u8,
    pub _unknown61: u8,
    pub _unknown62: u8,
    pub _unknown63: u8,
    pub _unknown64: u8,
    pub _unknown65: u8,
    pub _unknown66: u8,
    pub _unknown67: u8,
    pub _unknown68: u8,
    pub _unknown69: u8,
    pub _unknown70: u8,
    pub _unknown71: u8,
    pub _unknown72: u8,
    pub _unknown73: u8,
    pub _unknown74: u8,
    pub _unknown75: u8,
    pub _unknown76: u8,
    pub _unknown77: u8,
    pub _unknown78: u8,
    pub _unknown79: u8,
    pub _unknown80: u8,
    pub _unknown81: u8,
    pub DefaultSoundDisplayAt: u8,
    pub dItemMemorized: u8,
    pub dPending: u8,
    pub dIOutstanding: u8,
    pub dTOutstanding: u8,
    pub dMaxOutstanding: u8,
    pub dDrill: u8,
    pub dFI: f64,
    pub dItemBurden: f64,
    pub dTopicBurden: f64,
    pub dAIR: f64,
    pub dAL: f64,
    pub dAII: f64,
    pub dAILR: f64,
    pub dAIAF: f64,
    pub dAFI: f64,
    pub dATR: f64,
    pub dATI: f64,
    pub dATLR: f64,
    pub dATAF: f64,
    pub dLapses: u8,
    pub dReps: u8,
    pub HistoryQueue: u32,
    pub _unknown82: u8,
    pub _unknown83: u8,
    pub _unknown84: u8,
    pub _unknown85: u8,
    pub TextRegistry: u8,
    pub _unknown86: u8,
    pub _unknown87: u8,
    pub _unknown88: u8,
    pub _unknown89: u8,
    pub _unknown90: u8,
    pub _unknown91: u8,
    pub _unknown92: u8,
    pub _unknown93: u8,
    pub _unknown94: u8,
    pub _unknown95: u8,
    pub _unknown96: u8,
    pub _unknown97: u8,
    pub _unknown98: u8,
    pub _unknown99: u8,
    pub _unknown100: u8,
    pub _unknown101: u8,
    pub _unknown102: u8,
    pub _unknown103: u8,
    pub _unknown104: u8,
    pub _unknown105: u8,
    pub _unknown106: u8,
    pub _unknown107: u8,
    pub _unknown108: u8,
    pub _unknown109: u8,
    pub _unknown110: u8,
    pub _unknown111: u8,
    pub _unknown112: u8,
    pub _unknown113: u8,
    pub _unknown114: u8,
    pub _unknown115: u8,
    pub _unknown116: u8,
    pub _unknown117: u8,
    pub _unknown118: u8,
    pub _unknown119: u8,
    pub _unknown120: u8,
    pub _unknown121: u8,
    pub _unknown122: u8,
    pub _unknown123: u8,
    pub _unknown124: u8,
    pub _unknown125: u8,
    pub _unknown126: u8,
    pub _unknown127: u8,
    pub _unknown128: u8,
    pub _unknown129: u8,
    pub _unknown130: u8,
    pub _unknown131: u8,
    pub _unknown132: u8,
    pub _unknown133: u8,
    pub _unknown134: u8,
    pub _unknown135: u8,
    pub _unknown136: u8,
    pub _unknown137: u8,
    pub _unknown138: u8,
    pub _unknown139: u8,
    pub _unknown140: u8,
    pub _unknown141: u8,
    pub _unknown142: u8,
    pub _unknown143: u8,
    pub _unknown144: u8,
    pub _unknown145: u8,
    pub _unknown146: u8,
    pub _unknown147: u8,
    pub _unknown148: u8,
    pub _unknown149: u8,
    pub _unknown150: u8,
    pub _unknown151: u8,
    pub _unknown152: u8,
    pub _unknown153: u8,
    pub _unknown154: u8,
    pub _unknown155: u8,
    pub _unknown156: u8,
    pub HFile: [u8; 592],
    pub ItemInfoFile: [u8; 592],
    pub BurdenFile: [u8; 592],
    pub LocalComponentData: u32,
    pub SessionToday: u32,
    pub ItemInfoOpened: bool,
    pub ExtraInfoOpened: bool,
    pub HierarchyOpened: bool,
    pub BurdenOpened: bool,
    pub ArrangingFolders: bool,
    pub OptimizationRecordSize: u16,
    pub LowestMissedItemPriority: f32,
    pub _unknown157: u32,
    pub LowestMissedTopicPriority: f32,
    pub _unknown158: u32,
}

impl Default for Database {
    fn default() -> Database {
        Database {
            _unknown0: 0,
            hINF: 0,
            _unknown1: 0,
            Filespace: 0,
            Opened: false,
            DuringEntry: false,
            ResettingCollection: false,
            DrillRandomized: false,
            MidnightShiftWarningShown: false,
            DatabaseName: 0,
            FilterFile: 0,
            SystemTranslation: 0,
            Phonetics: 0,
            TextPath: 0,
            _unknown2: 0,
            _unknown3: 0,
            _unknown4: 0,
            _unknown5: 0,
            _unknown6: 0,
            _unknown7: 0,
            _unknown8: 0,
            _unknown9: 0,
            _unknown10: 0,
            _unknown11: 0,
            _unknown12: 0,
            _unknown13: 0,
            _unknown14: 0,
            _unknown15: 0,
            _unknown16: 0,
            _unknown17: 0,
            _unknown18: 0,
            _unknown19: 0,
            _unknown20: 0,
            _unknown21: 0,
            _unknown22: 0,
            _unknown23: 0,
            _unknown24: 0,
            _unknown25: 0,
            _unknown26: 0,
            _unknown27: 0,
            _unknown28: 0,
            _unknown29: 0,
            _unknown30: 0,
            _unknown31: 0,
            SourceTextPath: 0,
            PhoneticFont: 0,
            _unknown32: 0,
            _unknown33: 0,
            _unknown34: 0,
            _unknown35: 0,
            _unknown36: 0,
            _unknown37: 0,
            _unknown38: 0,
            _unknown39: 0,
            _unknown40: 0,
            Pending: ptr::null_mut(),
            Drill: ptr::null_mut(),
            PQueue: ptr::null_mut(),
            RepDates: ptr::null_mut(),
            Outstanding: ptr::null_mut(),
            OutstandingItems: ptr::null_mut(),
            OutstandingTopics: ptr::null_mut(),
            DataRecord: Default::default(),
            OptimizationRecord: ptr::null_mut(),
            _unknown41: 0,
            _unknown42: 0,
            _unknown43: 0,
            _unknown44: 0,
            _unknown45: 0,
            _unknown46: 0,
            _unknown47: 0,
            _unknown48: 0,
            _unknown49: 0,
            _unknown50: 0,
            _unknown51: 0,
            _unknown52: 0,
            _unknown53: 0,
            _unknown54: 0,
            _unknown55: 0,
            _unknown56: 0,
            _unknown57: 0,
            _unknown58: 0,
            _unknown59: 0,
            _unknown60: 0,
            _unknown61: 0,
            _unknown62: 0,
            _unknown63: 0,
            _unknown64: 0,
            _unknown65: 0,
            _unknown66: 0,
            _unknown67: 0,
            _unknown68: 0,
            _unknown69: 0,
            _unknown70: 0,
            _unknown71: 0,
            _unknown72: 0,
            _unknown73: 0,
            _unknown74: 0,
            _unknown75: 0,
            _unknown76: 0,
            _unknown77: 0,
            _unknown78: 0,
            _unknown79: 0,
            _unknown80: 0,
            _unknown81: 0,
            DefaultSoundDisplayAt: 0,
            dItemMemorized: 0,
            dPending: 0,
            dIOutstanding: 0,
            dTOutstanding: 0,
            dMaxOutstanding: 0,
            dDrill: 0,
            dFI: 0.0,
            dItemBurden: 0.0,
            dTopicBurden: 0.0,
            dAIR: 0.0,
            dAL: 0.0,
            dAII: 0.0,
            dAILR: 0.0,
            dAIAF: 0.0,
            dAFI: 0.0,
            dATR: 0.0,
            dATI: 0.0,
            dATLR: 0.0,
            dATAF: 0.0,
            dLapses: 0,
            dReps: 0,
            HistoryQueue: 0,
            _unknown82: 0,
            _unknown83: 0,
            _unknown84: 0,
            _unknown85: 0,
            TextRegistry: 0,
            _unknown86: 0,
            _unknown87: 0,
            _unknown88: 0,
            _unknown89: 0,
            _unknown90: 0,
            _unknown91: 0,
            _unknown92: 0,
            _unknown93: 0,
            _unknown94: 0,
            _unknown95: 0,
            _unknown96: 0,
            _unknown97: 0,
            _unknown98: 0,
            _unknown99: 0,
            _unknown100: 0,
            _unknown101: 0,
            _unknown102: 0,
            _unknown103: 0,
            _unknown104: 0,
            _unknown105: 0,
            _unknown106: 0,
            _unknown107: 0,
            _unknown108: 0,
            _unknown109: 0,
            _unknown110: 0,
            _unknown111: 0,
            _unknown112: 0,
            _unknown113: 0,
            _unknown114: 0,
            _unknown115: 0,
            _unknown116: 0,
            _unknown117: 0,
            _unknown118: 0,
            _unknown119: 0,
            _unknown120: 0,
            _unknown121: 0,
            _unknown122: 0,
            _unknown123: 0,
            _unknown124: 0,
            _unknown125: 0,
            _unknown126: 0,
            _unknown127: 0,
            _unknown128: 0,
            _unknown129: 0,
            _unknown130: 0,
            _unknown131: 0,
            _unknown132: 0,
            _unknown133: 0,
            _unknown134: 0,
            _unknown135: 0,
            _unknown136: 0,
            _unknown137: 0,
            _unknown138: 0,
            _unknown139: 0,
            _unknown140: 0,
            _unknown141: 0,
            _unknown142: 0,
            _unknown143: 0,
            _unknown144: 0,
            _unknown145: 0,
            _unknown146: 0,
            _unknown147: 0,
            _unknown148: 0,
            _unknown149: 0,
            _unknown150: 0,
            _unknown151: 0,
            _unknown152: 0,
            _unknown153: 0,
            _unknown154: 0,
            _unknown155: 0,
            _unknown156: 0,
            HFile: [0; 592],
            ItemInfoFile: [0; 592],
            BurdenFile: [0; 592],
            LocalComponentData: 0,
            SessionToday: 0,
            ItemInfoOpened: false,
            ExtraInfoOpened: false,
            HierarchyOpened: false,
            BurdenOpened: false,
            ArrangingFolders: false,
            OptimizationRecordSize: 0,
            LowestMissedItemPriority: 0.0,
            _unknown157: 0,
            LowestMissedTopicPriority: 0.0,
            _unknown158: 0,
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct Queue {
    _unknown: u32,
    TheType: u8,
    Size: u32,
    THugeArray: u32,
    Filename: u8,
}
