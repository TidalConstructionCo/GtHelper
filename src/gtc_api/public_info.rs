// Auto-generated API client from OpenAPI specification
// Add these dependencies to your Cargo.toml:
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// reqwest = { version = "0.11", features = ["json"] }

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ActivityEnum {
    #[serde(rename = "0 = Undefined")]
    Undefined,
    #[serde(rename = "1 = Online")]
    Online,
    #[serde(rename = "2 = UnderOneDay")]
    UnderOneDay,
    #[serde(rename = "3 = UnderThreeDays")]
    UnderThreeDays,
    #[serde(rename = "4 = Inactive")]
    Inactive,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BuildingProjectIdEnum {
    #[serde(rename = "0 = Undefined")]
    Undefined,
    #[serde(rename = "1 = GHeadquarters")]
    GHeadquarters,
    #[serde(rename = "2 = GAdministrationCenter")]
    GAdministrationCenter,
    #[serde(rename = "3 = GFlightCenter")]
    GFlightCenter,
    #[serde(rename = "4 = GWarehouse")]
    GWarehouse,
    #[serde(rename = "5 = GArcade")]
    GArcade,
    #[serde(rename = "6 = Observatory")]
    Observatory,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BuildingSlotStatus {
    #[serde(rename = "0 = Undefined")]
    Undefined,
    #[serde(rename = "1 = Empty")]
    Empty,
    #[serde(rename = "2 = Building")]
    Building,
    #[serde(rename = "3 = Debris")]
    Debris,
    #[serde(rename = "4 = Premium")]
    Premium,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BuildingTypeIdEnum {
    #[serde(rename = "0 = Undefined")]
    Undefined,
    #[serde(rename = "1 = Mine")]
    Mine,
    #[serde(rename = "2 = Smelter")]
    Smelter,
    #[serde(rename = "3 = Pump")]
    Pump,
    #[serde(rename = "4 = ColonyBarracks")]
    ColonyBarracks,
    #[serde(rename = "5 = GasCollector")]
    GasCollector,
    #[serde(rename = "6 = PrefabPlant")]
    PrefabPlant,
    #[serde(rename = "7 = Refinery")]
    Refinery,
    #[serde(rename = "8 = FoodProcessingPlant")]
    FoodProcessingPlant,
    #[serde(rename = "9 = Headquarters")]
    Headquarters,
    #[serde(rename = "10 = Farm")]
    Farm,
    #[serde(rename = "11 = BasicAssemblyPlant")]
    BasicAssemblyPlant,
    #[serde(rename = "12 = PolymerPlant")]
    PolymerPlant,
    #[serde(rename = "13 = ChemistryPlant")]
    ChemistryPlant,
    #[serde(rename = "14 = Warehouse")]
    Warehouse,
    #[serde(rename = "15 = TextileMill")]
    TextileMill,
    #[serde(rename = "16 = AdvancedAssemblyPlant")]
    AdvancedAssemblyPlant,
    #[serde(rename = "17 = Ranch")]
    Ranch,
    #[serde(rename = "18 = SurveyStation")]
    SurveyStation,
    #[serde(rename = "19 = Orchard")]
    Orchard,
    #[serde(rename = "20 = Laboratory")]
    Laboratory,
    #[serde(rename = "21 = ResidentialComplex")]
    ResidentialComplex,
    #[serde(rename = "22 = ComfortQuarters")]
    ComfortQuarters,
    #[serde(rename = "23 = StellarSuites")]
    StellarSuites,
    #[serde(rename = "24 = SemiconductorFoundry")]
    SemiconductorFoundry,
    #[serde(rename = "25 = ElectronicsFactory")]
    ElectronicsFactory,
    #[serde(rename = "26 = WeldingPlant")]
    WeldingPlant,
    #[serde(rename = "27 = QuarryComplex")]
    QuarryComplex,
    #[serde(rename = "28 = ScienceInstitute")]
    ScienceInstitute,
    #[serde(rename = "29 = AdvancedPrefabPlant")]
    AdvancedPrefabPlant,
    #[serde(rename = "30 = Shipyard")]
    Shipyard,
    #[serde(rename = "31 = MicroelectronicsFactory")]
    MicroelectronicsFactory,
    #[serde(rename = "32 = AdvancedMaterialsLab")]
    AdvancedMaterialsLab,
    #[serde(rename = "33 = ExoticMatterLab")]
    ExoticMatterLab,
    #[serde(rename = "34 = AquaponicsFarm")]
    AquaponicsFarm,
    #[serde(rename = "35 = RoboticsFacility")]
    RoboticsFacility,
    #[serde(rename = "36 = CulinaryStudio")]
    CulinaryStudio,
    #[serde(rename = "37 = ApexPrefabPlant")]
    ApexPrefabPlant,
    #[serde(rename = "38 = QuantumComputingCenter")]
    QuantumComputingCenter,
    #[serde(rename = "39 = AdvancedGasCollector")]
    AdvancedGasCollector,
    #[serde(rename = "40 = Foundry")]
    Foundry,
    #[serde(rename = "41 = NanomaterialLab")]
    NanomaterialLab,
    #[serde(rename = "42 = QuantumFactory")]
    QuantumFactory,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CashLogType {
    #[serde(rename = "2 = Sell")]
    Sell,
    #[serde(rename = "3 = Starting")]
    Starting,
    #[serde(rename = "4 = DailyMission")]
    DailyMission,
    #[serde(rename = "5 = Achievement")]
    Achievement,
    #[serde(rename = "7 = ProjectDonation")]
    ProjectDonation,
    #[serde(rename = "10 = Mission")]
    Mission,
    #[serde(rename = "40 = TradeContractSell")]
    TradeContractSell,
    #[serde(rename = "50 = AdminAdd")]
    AdminAdd,
    #[serde(rename = "100 = Buy")]
    Buy,
    #[serde(rename = "112 = CreateGuild")]
    CreateGuild,
    #[serde(rename = "140 = TradeContractBuy")]
    TradeContractBuy,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CompanyPerkIdEnum {
    #[serde(rename = "0 = None")]
    None,
    #[serde(rename = "1 = BaseWarehouses")]
    BaseWarehouses,
    #[serde(rename = "2 = ExchangeWarehouse")]
    ExchangeWarehouse,
    #[serde(rename = "3 = ShipWarehouse")]
    ShipWarehouse,
    #[serde(rename = "4 = ProductionSpeed")]
    ProductionSpeed,
    #[serde(rename = "5 = ShipDegradation")]
    ShipDegradation,
    #[serde(rename = "6 = ShipFuelSaving")]
    ShipFuelSaving,
    #[serde(rename = "7 = WorkforceConsumptionReduction")]
    WorkforceConsumptionReduction,
    #[serde(rename = "8 = OverheadReduction")]
    OverheadReduction,
    #[serde(rename = "9 = MissionCooldownReduction")]
    MissionCooldownReduction,
    #[serde(rename = "10 = MissionReward")]
    MissionReward,
    #[serde(rename = "11 = BuildingDegradationReduction")]
    BuildingDegradationReduction,
    #[serde(rename = "12 = HousingSize")]
    HousingSize,
    #[serde(rename = "13 = DonationMissionPreference")]
    DonationMissionPreference,
    #[serde(rename = "14 = ExploreMissionPreference")]
    ExploreMissionPreference,
    #[serde(rename = "15 = ProduceMissionPreference")]
    ProduceMissionPreference,
    #[serde(rename = "16 = DeliverPackagesMissionPreference")]
    DeliverPackagesMissionPreference,
    #[serde(rename = "17 = SourceAndDeliverMissionPreference")]
    SourceAndDeliverMissionPreference,
    #[serde(rename = "18 = SalvageMissionPreference")]
    SalvageMissionPreference,
    #[serde(rename = "19 = MegaShips")]
    MegaShips,
    #[serde(rename = "20 = StrictOversight")]
    StrictOversight,
    #[serde(rename = "21 = LaxOversight")]
    LaxOversight,
    #[serde(rename = "22 = BigMissions")]
    BigMissions,
    #[serde(rename = "23 = AdditionalMissionChoice")]
    AdditionalMissionChoice,
    #[serde(rename = "24 = ContractFeeReduction")]
    ContractFeeReduction,
    #[serde(rename = "25 = AdditionalBasePermits")]
    AdditionalBasePermits,
    #[serde(rename = "26 = EfficientOversight")]
    EfficientOversight,
    #[serde(rename = "27 = ProductionSpeedPerUnusedPermit")]
    ProductionSpeedPerUnusedPermit,
    #[serde(rename = "28 = MaximumMissionsLimit")]
    MaximumMissionsLimit,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContractStatusEnum {
    #[serde(rename = "0 = Pending")]
    Pending,
    #[serde(rename = "1 = Active")]
    Active,
    #[serde(rename = "2 = Completed")]
    Completed,
    #[serde(rename = "3 = Cancelled")]
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContractType {
    #[serde(rename = "1 = BuyMaterial")]
    BuyMaterial,
    #[serde(rename = "2 = SellMaterial")]
    SellMaterial,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FlightType {
    #[serde(rename = "0 = None")]
    None,
    #[serde(rename = "1 = Normal")]
    Normal,
    #[serde(rename = "2 = Emergency")]
    Emergency,
    #[serde(rename = "3 = Cancelled")]
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GuildMemberRankEnum {
    #[serde(rename = "0 = None")]
    None,
    #[serde(rename = "1 = RequestedToJoin")]
    RequestedToJoin,
    #[serde(rename = "2 = Invited")]
    Invited,
    #[serde(rename = "3 = Member")]
    Member,
    #[serde(rename = "4 = Officer")]
    Officer,
    #[serde(rename = "5 = Leader")]
    Leader,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MaterialIdEnum {
    #[serde(rename = "0 = Undefined")]
    Undefined,
    #[serde(rename = "1 = IronOre")]
    IronOre,
    #[serde(rename = "2 = IronBar")]
    IronBar,
    #[serde(rename = "3 = Concrete")]
    Concrete,
    #[serde(rename = "4 = Grain")]
    Grain,
    #[serde(rename = "5 = CopperOre")]
    CopperOre,
    #[serde(rename = "6 = CopperBar")]
    CopperBar,
    #[serde(rename = "7 = Oxygen")]
    Oxygen,
    #[serde(rename = "8 = Silica")]
    Silica,
    #[serde(rename = "9 = Milk")]
    Milk,
    #[serde(rename = "10 = Ale")]
    Ale,
    #[serde(rename = "11 = Water")]
    Water,
    #[serde(rename = "12 = BasicRations")]
    BasicRations,
    #[serde(rename = "13 = FineRations")]
    FineRations,
    #[serde(rename = "14 = LaboratorySuit")]
    LaboratorySuit,
    #[serde(rename = "15 = BasicExosuit")]
    BasicExosuit,
    #[serde(rename = "16 = DrinkingWater")]
    DrinkingWater,
    #[serde(rename = "17 = BasicTools")]
    BasicTools,
    #[serde(rename = "18 = AdvancedTools")]
    AdvancedTools,
    #[serde(rename = "19 = WeldingKit2")]
    WeldingKit2,
    #[serde(rename = "20 = Robot")]
    Robot,
    #[serde(rename = "21 = Coffee")]
    Coffee,
    #[serde(rename = "22 = Bioxene")]
    Bioxene,
    #[serde(rename = "23 = Tesserite")]
    Tesserite,
    #[serde(rename = "24 = Hydrogen")]
    Hydrogen,
    #[serde(rename = "25 = Polyethylene")]
    Polyethylene,
    #[serde(rename = "26 = BasicConstructionKit")]
    BasicConstructionKit,
    #[serde(rename = "27 = ConstructionTools")]
    ConstructionTools,
    #[serde(rename = "28 = Fruits")]
    Fruits,
    #[serde(rename = "29 = Vegetables")]
    Vegetables,
    #[serde(rename = "30 = Neoplast")]
    Neoplast,
    #[serde(rename = "31 = Carbon")]
    Carbon,
    #[serde(rename = "32 = Nitrogen")]
    Nitrogen,
    #[serde(rename = "33 = Glass")]
    Glass,
    #[serde(rename = "34 = Limestone")]
    Limestone,
    #[serde(rename = "35 = Steel")]
    Steel,
    #[serde(rename = "36 = Fertilizer")]
    Fertilizer,
    #[serde(rename = "37 = Cow")]
    Cow,
    #[serde(rename = "38 = Meat")]
    Meat,
    #[serde(rename = "39 = Cotton")]
    Cotton,
    #[serde(rename = "40 = UraniumOre")]
    UraniumOre,
    #[serde(rename = "41 = Flux")]
    Flux,
    #[serde(rename = "42 = AluminiumOre")]
    AluminiumOre,
    #[serde(rename = "43 = Aluminium")]
    Aluminium,
    #[serde(rename = "44 = Workwear")]
    Workwear,
    #[serde(rename = "45 = TitaniumOre")]
    TitaniumOre,
    #[serde(rename = "46 = Titanium")]
    Titanium,
    #[serde(rename = "47 = Furniture")]
    Furniture,
    #[serde(rename = "48 = Wood")]
    Wood,
    #[serde(rename = "49 = Leather")]
    Leather,
    #[serde(rename = "50 = Fabric")]
    Fabric,
    #[serde(rename = "51 = CoffeeBeans")]
    CoffeeBeans,
    #[serde(rename = "52 = ConstructionVehicle")]
    ConstructionVehicle,
    #[serde(rename = "53 = Rubber")]
    Rubber,
    #[serde(rename = "54 = CombustionEngine")]
    CombustionEngine,
    #[serde(rename = "55 = Motor")]
    Motor,
    #[serde(rename = "56 = Battery")]
    Battery,
    #[serde(rename = "57 = Gasoline")]
    Gasoline,
    #[serde(rename = "58 = Lubricant")]
    Lubricant,
    #[serde(rename = "59 = ElectronicCircuit")]
    ElectronicCircuit,
    #[serde(rename = "60 = Lithium")]
    Lithium,
    #[serde(rename = "61 = SulfuricAcid")]
    SulfuricAcid,
    #[serde(rename = "62 = CopperWiring")]
    CopperWiring,
    #[serde(rename = "63 = Electronics")]
    Electronics,
    #[serde(rename = "64 = ResearchData")]
    ResearchData,
    #[serde(rename = "65 = AdvancedResearchData")]
    AdvancedResearchData,
    #[serde(rename = "66 = OfficeSupplies")]
    OfficeSupplies,
    #[serde(rename = "67 = AeridiumOre")]
    AeridiumOre,
    #[serde(rename = "68 = Pipes")]
    Pipes,
    #[serde(rename = "69 = Argon")]
    Argon,
    #[serde(rename = "70 = Kryon")]
    Kryon,
    #[serde(rename = "71 = Coolant")]
    Coolant,
    #[serde(rename = "72 = Epoxy")]
    Epoxy,
    #[serde(rename = "73 = FissionFuel")]
    FissionFuel,
    #[serde(rename = "74 = Kevlar")]
    Kevlar,
    #[serde(rename = "75 = PlatinumOre")]
    PlatinumOre,
    #[serde(rename = "76 = Platinum")]
    Platinum,
    #[serde(rename = "77 = Graphene")]
    Graphene,
    #[serde(rename = "78 = CarbonNanotubes")]
    CarbonNanotubes,
    #[serde(rename = "79 = Aerogel")]
    Aerogel,
    #[serde(rename = "80 = Superconductors")]
    Superconductors,
    #[serde(rename = "81 = RadiationShielding")]
    RadiationShielding,
    #[serde(rename = "82 = LifeSupportSystem")]
    LifeSupportSystem,
    #[serde(rename = "83 = ReinforcedGlass")]
    ReinforcedGlass,
    #[serde(rename = "84 = ColorCompound")]
    ColorCompound,
    #[serde(rename = "85 = SpectraModulator")]
    SpectraModulator,
    #[serde(rename = "86 = MiningVehicle")]
    MiningVehicle,
    #[serde(rename = "87 = Drill")]
    Drill,
    #[serde(rename = "88 = Chicken")]
    Chicken,
    #[serde(rename = "89 = InsulationPanels")]
    InsulationPanels,
    #[serde(rename = "90 = PressureSealantKit")]
    PressureSealantKit,
    #[serde(rename = "91 = StructuralElements")]
    StructuralElements,
    #[serde(rename = "92 = BasicPrefabKit")]
    BasicPrefabKit,
    #[serde(rename = "93 = BasicAmenities")]
    BasicAmenities,
    #[serde(rename = "94 = AdvancedConstructionKit")]
    AdvancedConstructionKit,
    #[serde(rename = "95 = ApexStructuralElements")]
    ApexStructuralElements,
    #[serde(rename = "96 = AdvancedPrefabKit")]
    AdvancedPrefabKit,
    #[serde(rename = "97 = AdvancedAmenities")]
    AdvancedAmenities,
    #[serde(rename = "98 = ReinforcedTruss")]
    ReinforcedTruss,
    #[serde(rename = "99 = CompositeTruss")]
    CompositeTruss,
    #[serde(rename = "100 = AdvancedDrill")]
    AdvancedDrill,
    #[serde(rename = "101 = HydrogenGenerator")]
    HydrogenGenerator,
    #[serde(rename = "102 = ControlConsole")]
    ControlConsole,
    #[serde(rename = "103 = ShipInteriorKit")]
    ShipInteriorKit,
    #[serde(rename = "104 = BasicHullPlate")]
    BasicHullPlate,
    #[serde(rename = "105 = CargoBaySegment")]
    CargoBaySegment,
    #[serde(rename = "106 = FuelTankSegment")]
    FuelTankSegment,
    #[serde(rename = "107 = BasicPump")]
    BasicPump,
    #[serde(rename = "108 = WeldingKit")]
    WeldingKit,
    #[serde(rename = "109 = BasicFTLEmitter")]
    BasicFTLEmitter,
    #[serde(rename = "110 = HydrogenFuelCell")]
    HydrogenFuelCell,
    #[serde(rename = "111 = HeatShielding")]
    HeatShielding,
    #[serde(rename = "112 = AdvancedCircuitBoard")]
    AdvancedCircuitBoard,
    #[serde(rename = "113 = ShipRepairKit")]
    ShipRepairKit,
    #[serde(rename = "114 = QuadraniumHullPlate")]
    QuadraniumHullPlate,
    #[serde(rename = "115 = FTLFieldController")]
    FTLFieldController,
    #[serde(rename = "116 = SensorArray")]
    SensorArray,
    #[serde(rename = "117 = CoolingSystem")]
    CoolingSystem,
    #[serde(rename = "118 = BasicShipBridge")]
    BasicShipBridge,
    #[serde(rename = "119 = VRHeadset")]
    VRHeadset,
    #[serde(rename = "120 = CompositeShielding")]
    CompositeShielding,
    #[serde(rename = "121 = NanoweaveShielding")]
    NanoweaveShielding,
    #[serde(rename = "122 = Durablend")]
    Durablend,
    #[serde(rename = "123 = NeoplastSheet")]
    NeoplastSheet,
    #[serde(rename = "124 = Transistor")]
    Transistor,
    #[serde(rename = "125 = Chip")]
    Chip,
    #[serde(rename = "126 = SiliconWafer")]
    SiliconWafer,
    #[serde(rename = "127 = ApexResearchData")]
    ApexResearchData,
    #[serde(rename = "128 = Honeycaps")]
    Honeycaps,
    #[serde(rename = "129 = Sugar")]
    Sugar,
    #[serde(rename = "130 = Pie")]
    Pie,
    #[serde(rename = "131 = Eggs")]
    Eggs,
    #[serde(rename = "132 = ModernPrefabKit")]
    ModernPrefabKit,
    #[serde(rename = "133 = FissionReactor")]
    FissionReactor,
    #[serde(rename = "134 = AdvancedFTLEmitter")]
    AdvancedFTLEmitter,
    #[serde(rename = "135 = Aeridium")]
    Aeridium,
    #[serde(rename = "136 = TiridiumAlloy")]
    TiridiumAlloy,
    #[serde(rename = "137 = TiridiumHullPlate")]
    TiridiumHullPlate,
    #[serde(rename = "138 = AICore")]
    AICore,
    #[serde(rename = "139 = AdvancedShipBridge")]
    AdvancedShipBridge,
    #[serde(rename = "140 = Mainframe")]
    Mainframe,
    #[serde(rename = "141 = Nanopolyne")]
    Nanopolyne,
    #[serde(rename = "142 = Nanoweave")]
    Nanoweave,
    #[serde(rename = "143 = Drone")]
    Drone,
    #[serde(rename = "144 = ApexPrefabKit")]
    ApexPrefabKit,
    #[serde(rename = "145 = Cohesilite")]
    Cohesilite,
    #[serde(rename = "146 = OperatingSystem")]
    OperatingSystem,
    #[serde(rename = "147 = AI")]
    AI,
    #[serde(rename = "148 = AITrainingData")]
    AITrainingData,
    #[serde(rename = "149 = Antimatter")]
    Antimatter,
    #[serde(rename = "150 = AntimatterReactor")]
    AntimatterReactor,
    #[serde(rename = "151 = AntimatterContainment")]
    AntimatterContainment,
    #[serde(rename = "152 = HyperCoil")]
    HyperCoil,
    #[serde(rename = "153 = GourmetRations")]
    GourmetRations,
    #[serde(rename = "154 = ExoticSpices")]
    ExoticSpices,
    #[serde(rename = "155 = Lobster")]
    Lobster,
    #[serde(rename = "156 = Herbs")]
    Herbs,
    #[serde(rename = "157 = Rejuvaline")]
    Rejuvaline,
    #[serde(rename = "158 = Vitaqua")]
    Vitaqua,
    #[serde(rename = "159 = Quadranium")]
    Quadranium,
    #[serde(rename = "160 = SuperiorFTLEmitter")]
    SuperiorFTLEmitter,
    #[serde(rename = "161 = IndustrialMachinery")]
    IndustrialMachinery,
    #[serde(rename = "162 = Biopolyne")]
    Biopolyne,
    #[serde(rename = "163 = Nanobots")]
    Nanobots,
    #[serde(rename = "164 = QuantumResearchData")]
    QuantumResearchData,
    #[serde(rename = "165 = FiltrationSystem")]
    FiltrationSystem,
    #[serde(rename = "166 = T4ShipBridge")]
    T4ShipBridge,
    #[serde(rename = "167 = NeuralInterface")]
    NeuralInterface,
    #[serde(rename = "168 = T3RepairKit")]
    T3RepairKit,
    #[serde(rename = "169 = APU")]
    APU,
    #[serde(rename = "170 = Starglass")]
    Starglass,
    #[serde(rename = "171 = T4ShipElements")]
    T4ShipElements,
    #[serde(rename = "172 = Graphenium")]
    Graphenium,
    #[serde(rename = "173 = QuantumMainframe")]
    QuantumMainframe,
    #[serde(rename = "174 = FieldCooling")]
    FieldCooling,
    #[serde(rename = "175 = NutrientBlend")]
    NutrientBlend,
    #[serde(rename = "176 = Pack_Medicine")]
    PackMedicine,
    #[serde(rename = "177 = Pack_Food")]
    PackFood,
    #[serde(rename = "178 = Pack_ShipParts")]
    PackShipParts,
    #[serde(rename = "179 = Pack_Defense")]
    PackDefense,
    #[serde(rename = "180 = Pack_Habitats")]
    PackHabitats,
    #[serde(rename = "181 = Pack_Scientific")]
    PackScientific,
    #[serde(rename = "182 = Pack_Gifts")]
    PackGifts,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PBaseConsumptionMaterialModel {
    /// Material ID
    #[serde(rename = "matId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mat_id: Option<i32>,
    /// Is currently fulfilled
    #[serde(rename = "isEating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_eating: Option<bool>,
    /// Last updated date
    #[serde(rename = "luDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lu_date: Option<String>,
    /// Extra amount eaten last update date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer: Option<f32>,
    /// Amount consumed per day
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PBaseDetailResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "planetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planet_id: Option<i32>,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<i32>,
    #[serde(rename = "buildingSlots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_slots: Option<Vec<PBuildingSlotModel>>,
    #[serde(rename = "productionOrders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_orders: Option<Vec<PProductionOrder>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workforce: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse: Option<serde_json::Value>,
    /// How many slot expansions have been added to this base
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i32>,
    /// True when the base is in vacation mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vacation: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PBasePlanModel {
    /// Plan Id - planet ID this plan is for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Display name of the plan (max 40 characters) - Currently not used in UI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Number of base expansions to target (0-2). Has no effect if the live base already has this many or more expansions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i32>,
    /// Building slots that this plan affects; Omit any slot you want to leave unchanged
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<Vec<PBasePlanSlotModel>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PBasePlanSlotModel {
    /// Slot position ID (1-based)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Status of the slot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// Building type in this slot; Undefined (0) for no building;
    #[serde(rename = "buildingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_type: Option<i32>,
    /// Target level for the building
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PBasePreviewModel {
    /// Base ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "planetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planet_id: Option<i32>,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PBaseWorkforceModel {
    #[serde(rename = "baseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_id: Option<i32>,
    /// Workforce needed per type
    #[serde(rename = "workersNeeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers_needed: Option<Vec<i32>>,
    /// Workforce housing per type
    #[serde(rename = "workersHousing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers_housing: Option<Vec<i32>>,
    /// Working Workforce per type
    #[serde(rename = "workersCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers_count: Option<Vec<i32>>,
    #[serde(rename = "consumptionMaterials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_materials: Option<Vec<PBaseConsumptionMaterialModel>>,
    /// Workforce satisfaction per type (0-1)
    #[serde(rename = "workersSatisfaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers_satisfaction: Option<Vec<f32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PBuildingModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Building Type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    /// Condition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cond: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PBuildingProjectModel {
    /// Project type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// Current project level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    /// Materials stored in the project (includes both for construction and consumption)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub materials: Option<Vec<PMaterialAmountModel>>,
    /// Working percentage * 100 (8500 = 85%)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wp: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PBuildingProjectPreviewModel {
    /// Project type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// Current project level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PBuildingSlotModel {
    /// Building slot ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PCashLogModel {
    #[serde(rename = "exOrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ex_order_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// Other Company involved in the transaction
    #[serde(rename = "otherCompany")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_company: Option<serde_json::Value>,
    /// Quantity of items traded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    /// Cash change per unit in cents; Value is always positive; Types 100>= are negative cash changes
    #[serde(rename = "unitPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<i64>,
    #[serde(rename = "matId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mat_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PCompanyDetailModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Company rank in prestige leaderboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    /// Company prestige
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr: Option<i32>,
    /// Current activity status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<i32>,
    /// Company description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Total value of the company in credits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    /// Total workforce count (all workforce in all bases combined)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wf: Option<i32>,
    /// Date when the company was founded
    #[serde(rename = "fDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_date: Option<String>,
    /// Date when this company was last seen online; Returns current date for online companies
    #[serde(rename = "lastSeen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    /// Company logo as base64 string (custom format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ic: Option<String>,
    /// Guild ID, 0 if not in a guild
    #[serde(rename = "gId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_id: Option<i32>,
    /// Guild tag (Empty string if not in a guild)
    #[serde(rename = "gTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_tag: Option<String>,
    /// Rank within the guild
    #[serde(rename = "gRank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_rank: Option<i32>,
    /// List of planet IDs where the company has bases
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bases: Option<Vec<i32>>,
    /// Active exchange orders
    #[serde(rename = "exOrders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ex_orders: Option<Vec<PExchangeOrderModel>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PCompanyPerkModel {
    /// Perk enum ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Allocated Perk level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lvl: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PCompanyPreviewModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PCompanyRankModel {
    /// Company ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Company name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Company rank in prestige leaderboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    /// Company logo as base64 string (custom format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ic: Option<String>,
    /// Guild ID (0 if not in a guild)
    #[serde(rename = "gId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_id: Option<i32>,
    /// Guild tag (Empty string if not in a guild)
    #[serde(rename = "gTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_tag: Option<String>,
    /// Date when this company was last seen online; Returns current date for online companies
    #[serde(rename = "lastSeen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    /// Date when the company was founded
    #[serde(rename = "fDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_date: Option<String>,
    /// Company prestige
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr: Option<i32>,
    /// Total company value in credits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    /// Total workforce count (all workforce in all bases combined)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wf: Option<i32>,
    /// Number of bases owned - Deprecated field, use bases array length instead
    #[serde(rename = "basesC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bases_c: Option<i32>,
    /// List of planet IDs where the company has bases
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bases: Option<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PCreateWishlistRequest {
    /// Wishlist title (max 40 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Initial materials for the wishlist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mats: Option<Vec<PMaterialAmountModel>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PDeleteManyResult {
    /// IDs of successfully deleted wishlists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<Vec<i32>>,
    /// IDs that were not found or could not be deleted
    #[serde(rename = "notFound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_found: Option<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PExchangeAllMatDetailsResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub materials: Option<Vec<PExchangeMatDetailsResponseModel>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PExchangeMatDetailsResponseModel {
    /// Material ID
    #[serde(rename = "matId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mat_id: Option<i32>,
    /// Material Name
    #[serde(rename = "matName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mat_name: Option<String>,
    /// Current lowest price on exchange in cents (-1 if no orders available)
    #[serde(rename = "currentPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_price: Option<i32>,
    /// Average price from recent trades in cents (-1 if no trade history)
    #[serde(rename = "avgPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_price: Option<i32>,
    /// Total quantity available for sale (excluding Fed orders)
    #[serde(rename = "totalQtyAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_qty_available: Option<i32>,
    /// Active sell orders sorted by price (cheapest first, includes Fed orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<PExchangeOrderPublicModel>>,
    /// Average quantity sold per day (excluding Fed purchases from players)
    #[serde(rename = "avgQtySoldDaily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_qty_sold_daily: Option<f64>,
    /// Price history for the last 30 days (index 0 = today)
    #[serde(rename = "priceHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_history: Option<Vec<PExchangePriceHistoryModel>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PExchangeMaterialPriceModel {
    /// Material ID
    #[serde(rename = "matId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mat_id: Option<i32>,
    /// Material Name
    #[serde(rename = "matName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mat_name: Option<String>,
    /// Current lowest price on exchange in cents (-1 if no orders available)
    #[serde(rename = "currentPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_price: Option<i32>,
    /// Average price from recent trades in cents (-1 if no trade history)
    #[serde(rename = "avgPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_price: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PExchangeOrderModel {
    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Company ID of the owner
    #[serde(rename = "cId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_id: Option<i32>,
    /// Material ID
    #[serde(rename = "matId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mat_id: Option<i32>,
    /// Remaining quantity in the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<i32>,
    /// Total quantity originally posted in the order
    #[serde(rename = "qtyTot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty_tot: Option<i32>,
    /// Price per unit in cents
    #[serde(rename = "unitPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<i32>,
    /// User ID (company ID) of the owner - Deprecated, use cId instead)
    #[serde(rename = "uId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_id: Option<i32>,
    /// Remaining quantity - Deprecated, use qty instead
    #[serde(rename = "qRem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_rem: Option<i32>,
    /// Price per unit in cents - Deprecated, use unitPrice instead
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    /// Total quantity - Deprecated, use qtyTot instead
    #[serde(rename = "qTot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_tot: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PExchangeOrderPublicModel {
    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Company ID
    #[serde(rename = "cId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_id: Option<i32>,
    /// Company name
    #[serde(rename = "cName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_name: Option<String>,
    /// Price per unit in cents
    #[serde(rename = "unitPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<i32>,
    /// Remaining quantity in the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PExchangePriceHistoryModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Average traded price for this day in cents (excludes contracts)
    #[serde(rename = "avgPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_price: Option<i32>,
    /// Quantity sold this day (excluding Fed purchases)
    #[serde(rename = "qtySold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty_sold: Option<i32>,
    /// Quantity remaining at end of day (excluding Fed orders)
    #[serde(rename = "qtyRemaining")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty_remaining: Option<i32>,
    /// Quantity traded via contracts on this day
    #[serde(rename = "qtyC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty_c: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PExchangePricesResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<PExchangeMaterialPriceModel>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PGuildDetailModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Guild tag (2-3 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Guild description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Date when the guild was founded
    #[serde(rename = "fDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_date: Option<String>,
    /// Guild prestige
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr: Option<i32>,
    /// Guild rank in prestige leaderboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    /// Whether the guild is recruiting
    #[serde(rename = "isRec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rec: Option<bool>,
    /// List of guild members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<PGuildMemberModel>>,
    /// Guild building projects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<PBuildingProjectPreviewModel>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PGuildDonationModel {
    /// Donation ID for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Project type that received the donation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// DateTime when the donation was made
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Company that made the donation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<serde_json::Value>,
    /// Materials donated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mats: Option<Vec<PMaterialAmountModel>>,
    /// Prestige gained from this donation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PGuildInvitedModel {
    /// Company ID
    #[serde(rename = "cId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_id: Option<i32>,
    /// Company name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PGuildMemberModel {
    /// Company ID
    #[serde(rename = "cId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_id: Option<i32>,
    /// Company name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Rank within the guild (Leader = 5/Officer = 4/Member = 3)
    #[serde(rename = "gRank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_rank: Option<i32>,
    /// Total value of the company in credits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    /// Company prestige
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr: Option<i32>,
    /// Date when this company was last seen online; Returns current date for online companies
    #[serde(rename = "lastSeen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PGuildRankModel {
    /// Guild ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Guild name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Guild tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Guild rank in prestige leaderboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    /// Guild prestige
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr: Option<i32>,
    /// Headquarters level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hq: Option<i32>,
    /// Number of guild members
    #[serde(rename = "membersC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_c: Option<i32>,
    /// Date when the guild was founded
    #[serde(rename = "fDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PMaterialAmountModel {
    /// ID of the material
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub am: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PMyCompanyModel {
    /// My Company ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bases: Option<Vec<PBasePreviewModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ships: Option<Vec<PShipModel>>,
    /// Exchange Warehouse ID
    #[serde(rename = "exWhId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ex_wh_id: Option<i32>,
    /// Company cash balance in cents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash: Option<i64>,
    /// Company prestige
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr: Option<i32>,
    /// Rank of the company in prestige leaderboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stars: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technologies: Option<Vec<PTechnologyModel>>,
    /// Allocated perks; null if no perks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perks: Option<Vec<PCompanyPerkModel>>,
    /// Number of unlocked production order slots
    #[serde(rename = "poSlots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub po_slots: Option<i32>,
    /// Number of unlocked ship slots
    #[serde(rename = "shipSlots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_slots: Option<i32>,
    /// Date when the company was founded
    #[serde(rename = "fDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_date: Option<String>,
    /// Guild ID, 0 if not in a guild
    #[serde(rename = "gId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_id: Option<i32>,
    /// Rank within the guild
    #[serde(rename = "gRank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_rank: Option<i32>,
    /// Total value of the company in credits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PMyGuildDetailModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Guild tag (2-3 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Guild description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Guild board (members only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub board: Option<String>,
    /// Date when the guild was founded
    #[serde(rename = "fDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_date: Option<String>,
    /// Guild prestige
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr: Option<i32>,
    /// Guild rank in prestige leaderboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    /// Whether the guild is recruiting
    #[serde(rename = "isRec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rec: Option<bool>,
    /// List of guild members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<PGuildMemberModel>>,
    /// Companies invited to join the guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited: Option<Vec<PGuildInvitedModel>>,
    /// Companies that requested to join the guild
    #[serde(rename = "requestedJoins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_joins: Option<Vec<PGuildInvitedModel>>,
    /// Guild building projects (full details for members)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<PBuildingProjectModel>>,
    /// Multiplier for projects consumption amount
    #[serde(rename = "pConsMp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_cons_mp: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PProductionOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Recipe ID
    #[serde(rename = "rId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_id: Option<i32>,
    /// How many times to produce the recipe; 65535 = infinite production order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PProductionTask {
    /// Building Id
    #[serde(rename = "bId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_id: Option<i32>,
    /// Recipe Id
    #[serde(rename = "rId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_id: Option<i32>,
    /// Recipe Id - Old field, use rId
    #[serde(rename = "recipeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_id: Option<i32>,
    /// Date when production started
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Date when production was last updated
    #[serde(rename = "updD")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upd_d: Option<String>,
    /// Expected completion Date
    #[serde(rename = "comD")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub com_d: Option<String>,
    /// Part of the task completed at last update
    #[serde(rename = "updPart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upd_part: Option<f32>,
    /// Amount multiplier (both input and output, level 2 building produces 2x)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mul: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PRenameWishlistRequest {
    /// New wishlist title (max 40 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PShipBlueprintModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "reactorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactor_type: Option<i32>,
    #[serde(rename = "emitterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emitter_type: Option<i32>,
    #[serde(rename = "emittersCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emitters_count: Option<i32>,
    #[serde(rename = "cargoCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_capacity: Option<i32>,
    #[serde(rename = "tankType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tank_type: Option<i32>,
    #[serde(rename = "heatShielding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_shielding: Option<i32>,
    #[serde(rename = "radiationShielding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radiation_shielding: Option<i32>,
    #[serde(rename = "autoUnloadUnlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_unload_unlocked: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PShipFlightModel {
    /// Destination Planet ID
    #[serde(rename = "destPId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_p_id: Option<i32>,
    /// Date when the flight started
    #[serde(rename = "sDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_date: Option<String>,
    /// Date when the flight ends (arrival)
    #[serde(rename = "aDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_date: Option<String>,
    /// Fuel at the start of the flight
    #[serde(rename = "startFuel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_fuel: Option<f32>,
    /// Fuel at the end of the flight
    #[serde(rename = "arrivalFuel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_fuel: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// Auto unload on arrival
    #[serde(rename = "aUnload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_unload: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PShipModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Company ID
    #[serde(rename = "cId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_id: Option<i32>,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel: Option<f32>,
    /// Condition of the ship (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<f32>,
    /// Planet ID where the ship is currently located OR is traveling from
    #[serde(rename = "pId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_id: Option<i32>,
    /// Current flight information, null if stationary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight: Option<serde_json::Value>,
    #[serde(rename = "fuelCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_capacity: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PTechnologyModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PTradeContractModel {
    /// Contract ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Company that created the contract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<serde_json::Value>,
    /// Company that received the contract
    #[serde(rename = "otherCompany")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_company: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// Contract status (Pending = 0, Active = 1, Completed = 2, Cancelled = 3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// Planet ID where the contract is executed
    #[serde(rename = "pId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_id: Option<i32>,
    /// Material ID being traded
    #[serde(rename = "matId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mat_id: Option<i32>,
    /// Quantity to be traded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<i32>,
    /// Price per unit in cents (includes fees)
    #[serde(rename = "unitPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<i32>,
    /// Fee per unit in cents (included in UnitPrice)
    #[serde(rename = "unitFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_fee: Option<i32>,
    /// Date when the contract was created
    #[serde(rename = "aDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_date: Option<String>,
    /// Date when contract was Accepted/Filled/Canceled (date of last status change)
    #[serde(rename = "cDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_date: Option<String>,
    /// Current fulfillability state; null if not applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<serde_json::Value>,
    /// How many times the recurring contract can be fulfilled in a day (null if not a recurring contract)
    #[serde(rename = "fLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_limit: Option<i32>,
    /// How many times was this contract fulfilled in total.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ftotal: Option<i32>,
    /// How many times was the recurring contract fulfilled today (null if not a recurring contract)
    #[serde(rename = "fToday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_today: Option<i32>,
    /// For recurring contracts, the date until which the contract is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PTradeContractStateModel {
    /// Whether buyer has enough warehouse space
    #[serde(rename = "hasSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_space: Option<bool>,
    /// Whether seller has enough materials
    #[serde(rename = "hasMat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_mat: Option<bool>,
    /// Whether buyer has enough credits
    #[serde(rename = "hasCredits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credits: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PWarehouseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// Entity ID holding this warehouse: Base.Id when Type=1, Ship.Id when Type=2, otherwise 0
    #[serde(rename = "holderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_id: Option<i32>,
    /// Warehouse capacity in tonnes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap: Option<f32>,
    /// Materials stored in this warehouse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mats: Option<Vec<PMaterialAmountModel>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PWarehouseType {
    #[serde(rename = "0 = Unknown")]
    Unknown,
    #[serde(rename = "1 = Base")]
    Base,
    #[serde(rename = "2 = Ship")]
    Ship,
    #[serde(rename = "3 = Exchange")]
    Exchange,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PWishlistModel {
    /// 0= new wishlist, otherwise the wishlist id;
    /// 1-10 000 000= planet ID;
    /// 50 000 001+= custom wishlist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Wishlist title, null for planet wishlists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Materials in the wishlist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mats: Option<Vec<PMaterialAmountModel>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShipFTLType {
    #[serde(rename = "0 = Undefined")]
    Undefined,
    #[serde(rename = "1 = Basic")]
    Basic,
    #[serde(rename = "2 = AdvancedShortRange")]
    AdvancedShortRange,
    #[serde(rename = "3 = AdvancedEfficient")]
    AdvancedEfficient,
    #[serde(rename = "4 = AdvancedLongRange")]
    AdvancedLongRange,
    #[serde(rename = "5 = SuperiorShortRange")]
    SuperiorShortRange,
    #[serde(rename = "6 = SuperiorEfficient")]
    SuperiorEfficient,
    #[serde(rename = "7 = SuperiorLongRange")]
    SuperiorLongRange,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShipHeatShieldingType {
    #[serde(rename = "0 = Undefined")]
    Undefined,
    #[serde(rename = "1 = None")]
    None,
    #[serde(rename = "2 = Light")]
    Light,
    #[serde(rename = "3 = Heavy")]
    Heavy,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShipRadiationShieldingType {
    #[serde(rename = "0 = Undefined")]
    Undefined,
    #[serde(rename = "1 = None")]
    None,
    #[serde(rename = "2 = Light")]
    Light,
    #[serde(rename = "3 = Heavy")]
    Heavy,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShipReactorType {
    #[serde(rename = "0 = Undefined")]
    Undefined,
    #[serde(rename = "1 = HydrogenGenerator")]
    HydrogenGenerator,
    #[serde(rename = "2 = Fission")]
    Fission,
    #[serde(rename = "3 = Antimatter")]
    Antimatter,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShipTankSizeType {
    #[serde(rename = "0 = Undefined")]
    Undefined,
    #[serde(rename = "1 = Small")]
    Small,
    #[serde(rename = "2 = Medium")]
    Medium,
    #[serde(rename = "3 = Large")]
    Large,
}

#[derive(Debug, Serialize, Deserialize)]
#[repr(i32)]
pub enum SpecializationEnum {
    #[serde(rename = "0 = Undefined")]
    Undefined,
    #[serde(rename = "1 = Construction")]
    Construction,
    #[serde(rename = "2 = Manufacturing")]
    Manufacturing,
    #[serde(rename = "3 = Agriculture")]
    Agriculture,
    #[serde(rename = "4 = ResourceExtraction")]
    ResourceExtraction,
    #[serde(rename = "5 = Metallurgy")]
    Metallurgy,
    #[serde(rename = "6 = Chemistry")]
    Chemistry,
    #[serde(rename = "7 = Electronics")]
    Electronics,
    #[serde(rename = "8 = FoodProduction")]
    FoodProduction,
    #[serde(rename = "10 = Science")]
    Science,
}

// // API Client
// pub struct ApiClient {
//     base_url: String,
//     client: reqwest::Client,
// }

// impl ApiClient {
//     pub fn new(base_url: impl Into<String>) -> Self {
//         Self {
//             base_url: base_url.into(),
//             client: reqwest::Client::new(),
//         }
//     }
// }
