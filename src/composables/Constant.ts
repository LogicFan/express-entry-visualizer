export enum DrawCategory {
    // general
    GENERAL = "General",
    // 4 pathway
    PNP = "Province",
    CEC = "Inland",
    FSW = "Oversea",
    // occupation categories
    STEM = "STEM",
    HEALTH = "Health",
    FRENCH = "French",
    TRADE = "Trade",
    TRANSPORT = "Transport",
    AGRICULTURE = "Agriculture",
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
    name: DrawCategory;
    date: Date;
    pool: Array<Candidates>;
}

export function useCategoryColor(str: DrawCategory): string {
    if (str == DrawCategory.GENERAL) {
        return "#ECF0F1";
    } else if (str == DrawCategory.PNP) {
        return "#9B59B6";
    } else if (str == DrawCategory.CEC) {
        return "#E74C3C";
    } else if (str == DrawCategory.FSW) {
        return "#C0392B";
    } else if (str == DrawCategory.STEM) {
        return "#3498DB";
    } else if (str == DrawCategory.HEALTH) {
        return "#16A085";
    } else if (str == DrawCategory.FRENCH) {
        return "#D35400";
    } else if (str == DrawCategory.TRADE) {
        return "#7F8C8D";
    } else if (str == DrawCategory.TRANSPORT) {
        return "#F39C12";
    } else if (str == DrawCategory.AGRICULTURE) {
        return "#2ECC71";
    } else {
        return "#000000";
    }
}

export let IrccPlan = {
    2015: { min: 68000, max: 74000 },
    2016: { min: 54000, max: 59000 },
    2017: { min: 69600, max: 77300 },
    2018: { min: 72700, max: 78200 },
    2019: { min: 76000, max: 86000 },
    2020: { min: 88500, max: 100000 },
    2021: { min: 81000, max: 110250 },
    2022: { min: 52000, max: 64000 },
    2023: { min: 67750, max: 88000 },
    2024: { min: 90000, max: 116000 },
    2025: { min: 96500, max: 124000 },
    2026: { min: 96500, max: 124000 },
};
