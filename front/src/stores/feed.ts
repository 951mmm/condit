import { atom } from "jotai";

export const atomFeedQueryType = atom<
  "user" | "global" | "tag" | "private" | "favorites"
>("global");

export const atomTagName = atom("");

const atomUserId = atom("");

export const atomPageLink = atom("");

export const atomFeedQuery = atom<string>((get) => {
  const type = get(atomFeedQueryType);
  switch (type) {
    case "user":
      return "/feed?";
    case "global":
      return "?";
    case "tag":
      const tagName = get(atomTagName);
      return `?tag=${tagName}&`;
    default:
      const userId = get(atomUserId);
      switch (type) {
        case "private":
          return `?author=${userId}&`;
        case "favorites":
          return `?favorited=${userId}&`;
      }
  }
});

export const atomSetFeedQuery = atom(
  null,
  (
    get,
    set,
    [type, meta]: ["user" | "global" | "tag" | "private" | "favorites", string?]
  ) => {
    set(atomFeedQueryType, type);
    if (type === "user" || type === "global" || type === "tag") {
      set(atomPageLink, "/");
    }
    if (type === "tag") {
      set(atomTagName, meta!);
    }
    if (type === "private" || type === "favorites") {
      set(atomUserId, meta!);
    }
    if (type === "private") {
      set(atomPageLink, `/profile/${meta}`);
    }
    if (type === "favorites") {
      set(atomPageLink, `/profile/${meta}/favorites`);
    }
  }
);

export const atomQueryLimit = atom(10);

export const atomPage = atom(1);
