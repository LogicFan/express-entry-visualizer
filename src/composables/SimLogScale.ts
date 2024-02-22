import { LinearScale, Scale, CartesianScaleOptions, Ticks } from "chart.js";
import {
    finiteOrDefault,
    formatNumber,
    isFinite,
    _setMinAndMaxByKey,
    log10,
    sign,
} from "chart.js/helpers";

/** Copied from Chart.js Logarithmic Scale */
const log10Floor = (v) => Math.floor(log10(v));
const changeExponent = (v, m) => Math.pow(10, log10Floor(v) + m);

function isMajor(tickVal) {
    const remain = tickVal / Math.pow(10, log10Floor(tickVal));
    return remain === 1;
}

function steps(min, max, rangeExp) {
    const rangeStep = Math.pow(10, rangeExp);
    const start = Math.floor(min / rangeStep);
    const end = Math.ceil(max / rangeStep);
    return end - start;
}

function startExp(min, max) {
    const range = max - min;
    let rangeExp = log10Floor(range);
    while (steps(min, max, rangeExp) > 10) {
        rangeExp++;
    }
    while (steps(min, max, rangeExp) < 10) {
        rangeExp--;
    }
    return Math.min(rangeExp, log10Floor(min));
}

/**
 * Generate a set of logarithmic ticks
 * @param generationOptions the options used to generate the ticks
 * @param dataRange the range of the data
 * @returns {object[]} array of tick objects
 */
function generateTicks(generationOptions, { min, max }) {
    min = finiteOrDefault(generationOptions.min, min);
    const ticks = [];
    const minExp = log10Floor(min);
    let exp = startExp(min, max);
    let precision = exp < 0 ? Math.pow(10, Math.abs(exp)) : 1;
    const stepSize = Math.pow(10, exp);
    const base = minExp > exp ? Math.pow(10, minExp) : 0;
    const start = Math.round((min - base) * precision) / precision;
    const offset = Math.floor((min - base) / stepSize / 10) * stepSize * 10;
    let significand = Math.floor((start - offset) / Math.pow(10, exp));
    let value = finiteOrDefault(
        generationOptions.min,
        Math.round(
            (base + offset + significand * Math.pow(10, exp)) * precision
        ) / precision
    );
    while (value < max) {
        ticks.push({ value, major: isMajor(value), significand });
        if (significand >= 10) {
            significand = significand < 15 ? 15 : 20;
        } else {
            significand++;
        }
        if (significand >= 20) {
            exp++;
            significand = 2;
            precision = exp >= 0 ? 1 : precision;
        }
        value =
            Math.round(
                (base + offset + significand * Math.pow(10, exp)) * precision
            ) / precision;
    }
    const lastTick = finiteOrDefault(generationOptions.max, value);
    ticks.push({ value: lastTick, major: isMajor(lastTick), significand });

    return ticks;
}

export type SimLogScaleOptions = CartesianScaleOptions & {
    center: number;
    delta: number;
    ticks: {
        format: Intl.NumberFormatOptions;
    };
};

export class SimLogScale extends Scale<SimLogScaleOptions> {
    static id = "simlog";

    static defaults = {
        center: 0,
        delta: 0,
        ticks: {
            callback: Ticks.formatters.logarithmic,
            major: {
                enabled: true,
            },
        },
    };

    start: number;
    end: number;
    _startValue: number;
    _endValue: number;
    _valueRange: number;
    _splitValue: number;

    // the smallest difference that is not equal to center
    _delta: number;

    constructor(cfg) {
        super(cfg);

        this.start = undefined;
        this.end = undefined;
        this._startValue = undefined;
        this._endValue = undefined;
        this._splitValue = undefined;
        this._valueRange = 0;

        this._delta = undefined;
    }

    getDelta() {
        if (isFinite(this.options.delta) && this.options.delta != 0) {
            return this.options.delta;
        } else {
            return this._delta;
        }
    }

    parse(raw: number, index?: number) {
        const value = LinearScale.prototype.parse.apply(this, [raw, index]);

        if (isFinite(value)) {
            if (value != this.options.center) {
                if (!isFinite(this._delta)) {
                    this._delta = Math.abs(value - this.options.center);
                } else {
                    this._delta = Math.min(
                        this._delta,
                        Math.abs(value - this.options.center)
                    );
                }
            }
            return value;
        } else {
            return null;
        }
    }

    determineDataLimits() {
        const { min, max } = this.getMinMax(true);

        this.min = isFinite(min) ? min : 0;
        this.max = isFinite(max) ? max : 0;

        this.handleTickRangeOptions();

        console.log("min-max: " + this.min + ", " + this.max);
    }

    handleTickRangeOptions() {
        const { minDefined, maxDefined } = this.getUserBounds();
        // centered at 0
        let min = this.min - this.options.center;
        let max = this.max - this.options.center;

        // we want to update centered value to avoid log(0) issue
        const setMin = (v: number) => (min = minDefined && min != 0 ? min : v);
        const setMax = (v: number) => (max = maxDefined && max != 0 ? max : v);

        if (min === max) {
            if (min < 0) {
                // e.g. -10^2 to -10^0
                setMin(-changeExponent(-min, +1));
                setMax(-changeExponent(-min, -1));
            } else if (min > 0) {
                // e.g. 10^0 to 10^2
                setMin(changeExponent(min, -1));
                setMax(changeExponent(min, +1));
            } else {
                setMin(1);
                setMax(10);
            }
        }

        if (min == 0) {
            // max > min = 0
            setMin(changeExponent(this.getDelta(), -1));
        }

        if (max == 0) {
            // 0 = max > min
            setMax(-changeExponent(this.getDelta(), -1));
        }

        this.min = min + this.options.center;
        this.max = max + this.options.center;
    }

    buildTicks() {
        const opts = this.options;

        let min = this.min - opts.center;
        let max = this.max - opts.center;

        var ticks;

        if (min < 0 && max < 0) {
            let genOptions = { min: -max, max: -min };
            ticks = generateTicks(genOptions, genOptions)
                .reverse()
                .map((x) => Object.assign({}, x, { value: -x.value }));
        } else if (min < 0 && max > 0) {
            let pseudoZero = changeExponent(this.getDelta(), 1);
            let negOptions = { min: pseudoZero, max: -min };
            let neg = generateTicks(negOptions, negOptions)
                .reverse()
                .map((x) => Object.assign({}, x, { value: -x.value }));
            let posOptions = { min: pseudoZero, max: max };
            let pos = generateTicks(posOptions, posOptions);

            // insert 0 tick here
            if (neg.length > 0) {
                neg[neg.length - 1].major = false;
            }
            if (pos.length > 0) {
                pos[0].major = false;
            }
            ticks = neg
                .concat([{ value: 0, major: true, significand: 20 }])
                .concat(pos);
        } else if (min > 0 && max > 0) {
            let genOptions = { min: min, max: max };
            ticks = generateTicks(genOptions, genOptions);
        } else {
            ticks = [];
        }

        ticks = ticks.map((x) =>
            Object.assign({}, x, { value: x.value + opts.center })
        );

        // At this point, we need to update our max and min given the tick values,
        // since we probably have expanded the range of the scale
        if (opts.bounds === "ticks") {
            _setMinAndMaxByKey(ticks, this, "value");
        }

        if (opts.reverse) {
            ticks.reverse();

            this.start = this.max;
            this.end = this.min;
        } else {
            this.start = this.min;
            this.end = this.max;
        }

        console.log(ticks);

        return ticks;
    }

    getLabelForValue(value?: number) {
        return formatNumber(
            value,
            this.chart.options.locale,
            this.options.ticks.format
        );
    }

    computeTickLimit() {
        return Number.POSITIVE_INFINITY;
    }

    configure() {
        super.configure();

        let min = this.min - this.options.center;
        let max = this.max - this.options.center;

        if (sign(min) == sign(max)) {
            this._splitValue = null;
            if (min < 0) {
                this._startValue = -log10(-min);
                this._valueRange = log10(-min) - log10(-max);
            } else {
                this._startValue = log10(min);
                this._valueRange = log10(max) - log10(min);
            }
        } else {
            this._splitValue = log10(-min) / (log10(-min) + log10(max));
            this._startValue = -log10(-min);
            this._valueRange = log10(-min) + log10(max);
        }

        console.log(this._splitValue, this._startValue, this._valueRange);
    }

    getPixelForValue(value: number, _: number) {
        if (value === null || isNaN(value)) {
            return NaN;
        }

        let v0 = value - this.options.center;

        if (this._splitValue == null) {
            if (this._startValue > 0) {
                return this.getPixelForDecimal(
                    (log10(v0) - this._startValue) / this._valueRange
                );
            } else {
                return this.getPixelForDecimal(
                    (-log10(-v0) - this._startValue) / this._valueRange
                );
            }
        } else {
            if (v0 < 0) {
                return this.getPixelForDecimal(
                    (-log10(-v0) - this._startValue) / this._valueRange
                );
            } else if (v0 > 0) {
                return this.getPixelForDecimal(
                    (log10(v0) - this._startValue) / this._valueRange
                );
            } else {
                return this.getPixelForDecimal(this._splitValue);
            }
        }
    }

    getValueForPixel(pixel: number) {
        const decimal = this.getDecimalForPixel(pixel);

        if (decimal < this._splitValue) {
            return (
                -Math.pow(10, this._startValue + decimal * this._valueRange) +
                this.options.center
            );
        } else if (decimal > this._splitValue) {
            return (
                Math.pow(10, this._startValue + decimal * this._valueRange) +
                this.options.center
            );
        } else {
            return this.options.center;
        }
    }

    getPixelForTick(index: number) {
        return this.getPixelForValue(this.ticks[index].value, 0);
    }

    // // Adds labels to objects in the ticks array. The default implementation simply calls this.options.ticks.callback(numericalTick, index, ticks);
    // generateTickLabels() {}
}
