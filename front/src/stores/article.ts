import { atom } from "jotai";
import { Article } from "../api/article";
import { Comment } from "../api/comment";

export const atomArticle = atom<Article.Article | undefined>(undefined);

export const atomComments = atom<Comment.Comment[]>([]);