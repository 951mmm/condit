import { atom } from "jotai";
import { User } from "../api/user";

export const atomUser = atom<Omit<User.User, "token">>({
  username: "",
  email: "",
  bio: "",
  image: "",
});

export const atomIsLogin = atom(false);

export const atomError = atom({
  index: 0,
  msgs: [""]
})