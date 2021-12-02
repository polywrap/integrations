import {
  Input_createTickListDataProvider,
  Input_getTick,
  Input_nextInitializedTickWithinOneWord,
  NextTickResult,
  Tick,
  TickListDataProvider,
} from "./w3";
import * as TickListUtils from "./tickListUtils";

/**
 * constructs and validates a TickListDataProvider
 */
export function createTickListDataProvider(
  input: Input_createTickListDataProvider
): TickListDataProvider {
  const ticks: Tick[] = input.ticks;
  const tickSpacing: i32 = input.tickSpacing;
  TickListUtils.validateTickList({ ticks: ticks, tickSpacing: tickSpacing });
  return {
    ticks: ticks,
  };
}

/**
 * returns tick at requested index
 */
export function getTick(input: Input_getTick): Tick {
  const tickIndex: i32 = input.tickIndex;
  const ticks: Tick[] = input.tickDataProvider.ticks;
  return TickListUtils.findTick(ticks, tickIndex);
}

/**
 * returns next initialized tick, or max or min tick. Returns true if a tick is found at index
 */
export function nextInitializedTickWithinOneWord(
  input: Input_nextInitializedTickWithinOneWord
): NextTickResult {
  const tick: i32 = input.tick;
  const lte: boolean = input.lte;
  const tickSpacing: i32 = input.tickSpacing;
  const ticks: Tick[] = input.tickDataProvider.ticks;

  const compressed: i32 = tick / tickSpacing;

  if (lte) {
    const wordPos: i32 = compressed >> 8;
    const minimum: i32 = (wordPos << 8) * tickSpacing;

    if (TickListUtils.tickIsBelowSmallest({ ticks: ticks, tick: tick })) {
      return {
        index: minimum,
        found: false,
      };
    }

    const index: i32 = TickListUtils.nextInitializedTick({
      ticks: ticks,
      tick: tick,
      lte: lte,
    }).index;
    const nextInitializedTick: i32 = index > minimum ? index : minimum;
    return {
      index: nextInitializedTick,
      found: nextInitializedTick == index,
    };
  } else {
    const wordPos = (compressed + 1) >> 8;
    const maximum = ((wordPos + 1) << 8) * tickSpacing - 1;

    if (TickListUtils.tickIsAtOrAboveLargest({ ticks: ticks, tick: tick })) {
      return {
        index: maximum,
        found: false,
      };
    }

    const index: i32 = TickListUtils.nextInitializedTick({
      ticks: ticks,
      tick: tick,
      lte: lte,
    }).index;
    const nextInitializedTick = maximum < index ? maximum : index;
    return {
      index: nextInitializedTick,
      found: nextInitializedTick == index,
    };
  }
}
