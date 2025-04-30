/**
 * Represents a page of data.
 * The next page can be accessed using the cursor field, if present.
 */
export interface Page<T> {
  data: T[],
  cursor?: string,
}
