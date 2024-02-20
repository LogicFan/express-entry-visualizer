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
    min: number;
    max: number;
    count: number;
}

export interface Draw {
    id: number;
    crs: number;
    size: number;
    name: DrawName;
    date: Date;
    pool: Array<Candidates>;
}

export function useDrawColor(str: DrawName): string {
    if (str == DrawName.GENERAL) {
        return "#ECF0F1";
    } else if (str == DrawName.PNP) {
        return "#9B59B6";
    } else if (str == DrawName.CEC) {
        return "#E74C3C";
    } else if (str == DrawName.FSW) {
        return "#C0392B";
    } else if (str == DrawName.FST) {
        return "#95A5A6";
    } else if (str == DrawName.STEM_2023) {
        return "#3498DB";
    } else if (str == DrawName.HEALTH_2023) {
        return "#16A085";
    } else if (str == DrawName.FRENCH_2023) {
        return "#D35400";
    } else if (str == DrawName.TRADE_2023) {
        return "#7F8C8D";
    } else if (str == DrawName.TRANSPORT_2023) {
        return "#F39C12";
    } else if (str == DrawName.AGRICULTURE_2023) {
        return "#2ECC71";
    } else {
        return "#000000";
    }
}
