import { random, inv, matrix } from 'mathjs';

/**
 *
 * @param size {number}
 * @returns {Array<Array<number>>}
 */
export function invertRandomMatrix(size) {
  if (size < 1) {
    throw new Error(
      `Matrix size must be greater or equal to "1", but was ${size}.`,
    );
  }

  const randomMatrix = matrix(
    Array.from({ length: size }, () =>
      Array.from({ length: size }, () => random()),
    ),
  );

  return inv(randomMatrix).toArray();
}
