import { GET } from "./config";

export namespace Tag {
  export namespace list {
    export function handler(): Promise<{tags: string[]}> {
      return GET("/tags");
    }
  }
}