export interface IAction<T> {
  type: keyof T,
  payload: typeof T[keyof T]
}