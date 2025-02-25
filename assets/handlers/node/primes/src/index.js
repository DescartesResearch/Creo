/**
 *
 * @returns {Generator<number, void, *>}
 */
function* generatePrimes() {
  const map = new Map();
  let q = 2;

  while (true) {
    if (!map.has(q)) {
      yield q;

      map.set(q * q, [q]);
    } else {
      for (const p of map.get(q)) {
        const next = p + q;

        if (!map.has(next)) {
          map.set(next, []);
        }

        map.get(next).push(p);
      }
      map.delete(q);
    }
    ++q;
  }
}

/**
 *
 * @param n {number}
 * @returns {Array<number>}
 */
export function generateFirstPrimes(n) {
  if (n < 1) {
    throw new Error(`"n" must be greater or equal to "1", but was ${n}.`);
  }

  const generator = generatePrimes();

  return Array.from({ length: n }, () => generator.next().value);
}
