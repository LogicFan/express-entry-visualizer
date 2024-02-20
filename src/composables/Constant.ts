export enum DrawName {
    // general
    GENERAL = "General",
    // 4 pathway
    PNP = "PNP",
    CEC = "CEC",
    FSW = "FSW",
    FST = "FST",
    // 2023 occupation category
    STEM_2023 = "STEM 2023-1",
    HEALTH_2023 = "Health 2023-1",
    FRENCH_2023 = "French 2023-1",
    TRADE_2023 = "Trade 2023-1",
    TRANSPORT_2023 = "Transport 2023-1",
    AGRICULTURE_2023 = "Agriculture 2023-1",
    // unknown
    UNKNOWN = "Unknown",
}

export interface Candidates {
    min: Number;
    max: Number;
    count: Number;
}

export interface Draw {
    id: Number;
    crs: Number;
    size: Number;
    name: DrawName;
    date: Number; // unix timestamp
    pool: Array<Candidates>;
}
