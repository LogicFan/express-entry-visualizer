import { LinearScale, LinearScaleOptions } from "chart.js";

const f = (x: number, n: number) => {
    return Math.pow(Math.max(0, x), n) - Math.pow(Math.max(0, -x), n);
};

const fNorm = (x: number, range: { min: number; max: number }) => {
    let r = range.max - range.min;
    return ((x - range.min) / r) * 2 - 1;
};

const rNorm = (x: number, range: { min: number; max: number }) => {
    let r = range.max - range.min;
    return (x + 1) * r * 0.5 + range.min;
};

export type FocusScaleOptions = LinearScaleOptions & {
    level: number;
};

export class FocusScale extends LinearScale<FocusScaleOptions> {
    static id = "focus";

    static defaults = {
        level: 1,
    };

    start: number;
    end: number;
    _startValue: number;
    _endValue: number;
    _valueRange: number;

    constructor(cfg) {
        super(cfg);

        this.start = undefined;
        this.end = undefined;
        this._startValue = undefined;
        this._endValue = undefined;
        this._valueRange = 0;
    }

    fTransform(x: number) {
        let range = { min: this.min, max: this.max };
        let a = fNorm(x, range);
        let b = f(a, this.options.level);
        let c = rNorm(b, range);
        return Math.round(c);
    }

    rTransform(x: number) {
        let range = { min: this.min, max: this.max };
        let a = fNorm(x, range);
        let b = f(a, 1 / this.options.level);
        let c = rNorm(b, range);
        return Math.round(c);
    }

    buildTicks() {
        let ticks = super.buildTicks();

        return ticks.map((e) => Object.assign({}, e, { value: this.fTransform(e.value) }));
    }

    getPixelForValue(value: number, index: number) {
        return super.getPixelForValue(this.rTransform(value), index);
    }

    getValueForPixel(pixel: number) {
        return super.getValueForPixel(this.fTransform(pixel));
    }
}
