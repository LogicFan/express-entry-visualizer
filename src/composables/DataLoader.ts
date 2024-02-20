import { DrawCategory, Draw, Candidates } from "./Constant.ts";

interface RawData {
    classes: string;
    rounds: Array<RawDataItem>;
}

interface RawDataItem {
    DrawText1: string;
    dd1: string;
    dd2: string;
    dd3: string;
    dd4: string;
    dd5: string;
    dd6: string;
    dd7: string;
    dd8: string;
    dd9: string;
    dd10: string;
    dd11: string;
    dd12: string;
    dd13: string;
    dd14: string;
    dd15: string;
    dd16: string;
    dd17: string;
    dd18: string;
    drawCRS: string;
    drawCutOff: string;
    drawDate: string;
    drawDateFull: string;
    drawDateTime: string;
    drawDistributionAsOn: string;
    drawName: string;
    drawNumber: string;
    drawNumberURL: string;
    drawSize: string;
    drawText2: string;
    mitext: string;
}

async function fetchRawData(): Promise<RawData> {
    const rawDataUrl = "https://www.canada.ca/content/dam/ircc/documents/json/ee_rounds_123_en.json";

    const response = await fetch(rawDataUrl);

    if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
    }

    return response.json();
}

export async function useData(): Promise<Array<Draw>> {
    const data = await fetchRawData();

    console.log(data);

    return data["rounds"].map(function (e) {
        return {
            id: parseInt(e.drawNumber.replace(/,/g, "")),
            crs: parseInt(e.drawCRS.replace(/,/g, "")),
            size: parseInt(e.drawSize.replace(/,/g, "")),
            name: convertDrawName(e.drawName),
            date: new Date(Date.parse(e.drawDateFull)),
            pool: createPool([e.dd1, e.dd2, e.dd4, e.dd5, e.dd6, e.dd7, e.dd8, e.dd10, e.dd11, e.dd12, e.dd13, e.dd14, e.dd15, e.dd16, e.dd17]),
        };
    });
}

function convertDrawName(rawName: string): DrawCategory {
    if (rawName == "No Program Specified") {
        return DrawCategory.GENERAL; // discontinued, replaced by General
    } else if (rawName == "General") {
        return DrawCategory.GENERAL;
    } else if (rawName == "Provincial Nominee Program") {
        return DrawCategory.PNP;
    } else if (rawName == "Canadian Experience Class") {
        return DrawCategory.CEC;
    } else if (rawName == "Federal Skilled Worker") {
        return DrawCategory.FSW;
    } else if (rawName == "Federal Skilled Trades") {
        return DrawCategory.TRADE;
    } else if (rawName == "STEM occupations (2023-1)") {
        return DrawCategory.STEM;
    } else if (rawName == "Healthcare occupations (2023-1)") {
        return DrawCategory.HEALTH;
    } else if (rawName == "French language proficiency (2023-1)") {
        return DrawCategory.FRENCH; // discontinued, replaced by French 2024-1
    } else if (rawName == "French language proficiency (2024-1)") {
        return DrawCategory.FRENCH;
    } else if (rawName == "Trade occupations (2023-1)") {
        return DrawCategory.TRADE;
    } else if (rawName == "Transport occupations (2023-1)") {
        return DrawCategory.TRANSPORT;
    } else if (rawName == "Agriculture and agri-food occupations (2023-1)") {
        return DrawCategory.AGRICULTURE;
    } else {
        return DrawCategory.UNKNOWN;
    }
}

function createPool(rawPoolData: Array<string>): Array<Candidates> {
    const minArr = [0, 301, 351, 401, 411, 421, 431, 441, 451, 461, 471, 481, 491, 501, 601];
    const maxArr = [300, 350, 400, 410, 420, 430, 440, 450, 460, 470, 480, 490, 500, 600, 1200];
    const countArr = rawPoolData.map((e) => parseInt(e.replace(/,/g, "")));

    if (countArr.reduce((x, y) => x + y, 0) == 0) {
        return [];
    } else {
        return countArr.map(function (e, i) {
            return {
                min: minArr[i],
                max: maxArr[i],
                count: e,
            };
        });
    }
}
