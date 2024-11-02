import { atom } from "jotai";

type FeedType = "user" | "global" | "tag" | "private" | "favorites";

export const atomFeedQueryType = atom<FeedType>("global");

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
    _,
    set,
    [type, meta]: [FeedType, string?]
  ) => {
    set(atomFeedQueryType, type);
    if (type === "tag") {
      set(atomTagName, meta!);
      set(atomPageLink, "/");
    } else if (["private", "favorites"].includes(type)) {
      set(atomUserId, meta!);
      set(
        atomPageLink,
        `/profile/${meta}/${type === "favorites" ? "favorites" : ""}`
      );
    } else {
      set(atomPageLink, "/");
    }
  }
);
export const atomQueryLimit = atom(10);

export const atomPage = atom(1);

export const atomFavorTrigger = atom(false);

export const atomQueryString = atom("");
