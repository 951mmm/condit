import { DELETE, GET, POST } from "./config";

export namespace Profile {
  export interface Profile {
    username: string;
    bio: string;
    image: string;
    following: boolean;
  }
  export namespace get {
    export function handler(username: string): Promise<{ profile: Profile }> {
      return GET(`/profiles/${username}`);
    }
  }

  export namespace follow {
    export function handler(username: string) {
      return POST(`/profiles/${username}/follow`);
    }
  }

  export namespace disFollow {
    export function handler(username: string) {
      return DELETE(`/profiles/${username}/follow`);
    }
  }
}
