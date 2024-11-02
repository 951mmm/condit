import { GET } from "./config";

export namespace Tag {
  export namespace list {
    export function handler(queryString: String): Promise<{tags: string[]}> {
      return GET(`/tags?query_string=${queryString}`);
    }
  }
}