/**
 *
 * @param min {number}
 * @param max {number}
 * @returns {Generator<number, void, *>}
 */
export function* yieldRandomNumer(min, max) {
  while (true) {
    yield Math.floor(Math.random() * (max - min + 1)) + min;
  }
}

/**
 *
 * @param n {number}
 * @param min {number}
 * @param max {number}
 */
export function generateRandomNumbers(n, min, max) {
  if (n < 1) {
    throw new Error(`"n" must be greater or equal to "1", but was ${n}.`);
  }

  if (min > max) {
    min += max;
    max = min - max;
    min = min - max;
  }

  const generator = yieldRandomNumer(min, max);

  return Array.from({ length: n }, () => generator.next().value);
}
