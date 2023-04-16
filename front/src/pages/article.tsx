import { ChangeEvent, FormEvent, useEffect, useState } from "react";
import { Link, useNavigate, useParams } from "react-router-dom";
import { useAtom, useAtomValue } from "jotai";
import { atomIsLogin, atomUser } from "../stores/auth";
import { Helmet, HelmetProvider } from "react-helmet-async";
import { atomArticle, atomComments } from "../stores/article";
import { ArticleMeta } from "../components/article/article-meta";
import { Article as ArticleApi } from "../api/article";
import { errHandler } from "../utils";
import { ReactMarkdown } from "react-markdown/lib/react-markdown";
import remarkGfm from "remark-gfm";
import { ArticleTag } from "../components/common/article-tag";
import { Comment as CommentApi } from "../api/comment";
import { Comment } from "../components/article/comment";

export function Article() {
  // ANCHOR state
  const [loading, setLoading] = useState(false);
  const [comment, setComment] = useState("");
  const [commentLoading, setCommentLoading] = useState(false);

  // ANCHOR store
  const [article, setArticle] = useAtom(atomArticle);
  const [comments, setComments] = useAtom(atomComments);
  const isLogin = useAtomValue(atomIsLogin);
  const user = useAtomValue(atomUser);
  const { URLSlug } = useParams();
  const navigate = useNavigate();

  // ANCHOR event
  async function onCommentPublish(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    setCommentLoading(true);
    try {
      const data = await CommentApi.post.handler(URLSlug!, {
        comment: {
          body: comment,
        },
      });
      setComments((comments) => [data.comment, ...comments]);
      setComment("");
    } catch (e) {
      errHandler(e);
    }
    setCommentLoading(false);
  }

  function onChange(e: ChangeEvent<HTMLTextAreaElement>) {
    const { value } = e.target;
    setComment(value);
  }

  // ANCHOR effect
  useEffect(() => {
    async function initArticle() {
      setLoading(true);
      try {
        const data = await ArticleApi.get.handler(URLSlug!);
        setArticle(data.article);
      } catch (e) {
        errHandler(e);
        navigate("/", { replace: true });
      }
      setLoading(false);
    }

    initArticle();
  }, [URLSlug, user.username]);

  useEffect(() => {
    async function initComments() {
      const data = await CommentApi.list.handler(URLSlug!);
      setComments(data.comments);
    }

    initComments();
  }, [URLSlug]);

  if (loading) return <p>loading...</p>;
  return (
    <>
      <HelmetProvider>
        <Helmet>
          <title>{article?.title}</title>
        </Helmet>
      </HelmetProvider>
      <div className="article-page">
        <div className="banner">
          <div className="container">
            <h1>{article?.title}</h1>
            <ArticleMeta />
          </div>
        </div>
        <div className="container page">
          <div className="row article-content">
            <div className="col-md-12">
              <ReactMarkdown
                children={article?.body!}
                remarkPlugins={[remarkGfm]}
              />
            </div>
          </div>
          <ul className="tag-list">
            {article?.tagList!.map((tag, index) => (
              <ArticleTag key={index} name={tag} />
            ))}
          </ul>
          <hr />
          <div className="article-actions">
            <ArticleMeta />
          </div>

          <div className="row">
            <div className="col-xs-12 col-md-8 offset-md-2">
              {isLogin ? (
                <form className="card comment-form" onSubmit={onCommentPublish}>
                  <div className="card-block">
                    <textarea
                      rows={3}
                      placeholder="Write a comment..."
                      value={comment}
                      onChange={onChange}
                      className="form-control"
                    />
                  </div>
                  <div className="card-footer">
                    <img
                      src={user.image}
                      alt={user.username}
                      className="comment-author-img"
                    />
                    <button
                      className="btn btn-sm btn-primary"
                      disabled={commentLoading}
                    >
                      Post Comment
                    </button>
                  </div>
                </form>
              ) : (
                <p>
                  <Link to="/login">Sign in</Link>
                  or <Link to="/register">Sign up</Link>
                  to add comments on the articles.
                </p>
              )}

              {comments.map((comment) => (
                <Comment
                  key={comment.id}
                  comment={comment}
                  username={user.username}
                  slug={URLSlug!}
                />
              ))}
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
