import { useAtom, useAtomValue } from "jotai";
import {
  ChangeEvent,
  FormEvent,
  KeyboardEvent,
  useEffect,
  useState,
} from "react";
import { atomIsLogin, atomUser } from "../stores/auth";
import { useNavigate, useParams } from "react-router-dom";
import { Helmet, HelmetProvider } from "react-helmet-async";
import { RemovableTag } from "../components/editor/removable-tag";
import { atomTagList } from "../stores/editor";
import { Article as ArticleApi } from "../api/article";
import { errHandler } from "../utils";
import { ReactMarkdown } from "react-markdown/lib/react-markdown";
import remarkGfm from "remark-gfm";
import useMeasure from "react-use-measure";

interface EditorProps {
  type: "new" | "update";
}

export function Editor({ type }: EditorProps) {
  // ANCHOR state
  const [editor, setEditor] = useState({
    title: "",
    description: "",
    body: "",
  });
  const [tag, setTag] = useState("");
  const [preview, setPreview] = useState(false);
  const [error, setError] = useState({
    index: 0,
    msg: [],
  });
  const errMsg = error.msg.join(" ");
  const [loading, setLoading] = useState(false);
  const [publishLoading, setPublishLoading] = useState(false);
  const [previewHeight, setPreviewHeight] = useState(0);
  const [previewWidth, setPreviewWidth] = useState(0);

  // ANCHOR store
  const isLogin = useAtomValue(atomIsLogin);
  const user = useAtomValue(atomUser);
  const [tagList, setTagList] = useAtom(atomTagList);
  const navigate = useNavigate();
  const { URLSlug } = useParams();
  const [ref, bounds] = useMeasure();

  // ANCHOR event
  function onChange(e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) {
    const { name, value } = e.target;
    if (name === "tag") {
      setTag(value);
    } else {
      setEditor((editor) => ({
        ...editor,
        [name]: value,
      }));
    }
  }

  function addTag(tag: string) {
    if (!tagList.includes(tag)) {
      setTagList((tagList) => [...tagList, tag]);
      setTag("");
    }
  }

  function onCreateTag(e: KeyboardEvent<HTMLInputElement>) {
    if (e.key === "Enter") {
      e.preventDefault();
      addTag(tag);
    }
  }

  async function publishArticle() {
    try {
      let data: { article: ArticleApi.Article };
      switch (type) {
        case "new":
          data = await ArticleApi.post.handler({
            article: {
              ...editor,
              tagList,
            },
          });
          break;
        case "update": {
          data = await ArticleApi.put.handler(URLSlug!, {
            article: {
              ...editor,
              tagList,
            },
          });
        }
      }
      navigate(`/article/${data.article.slug}`);
    } catch (e: any) {
      if (e.response.status === 422) {
        const errMsg = e.response.data.errors;
        errMsg.title &&
          setError({
            index: 1,
            msg: errMsg.title,
          });
        errMsg.description &&
          setError({
            index: 2,
            msg: errMsg.description,
          });
        errMsg.body &&
          setError({
            index: 3,
            msg: errMsg.body,
          });
      }
    }
  }

  // ANCHOR effect
  useEffect(() => {
    if (type === "update") {
      async function initArticle() {
        setLoading(true);
        try {
          const data = await ArticleApi.get.handler(URLSlug!);
          const { article } = data;
          if (!isLogin || article.author.username !== user.username) {
            navigate("/", { replace: true });
            return;
          }

          setEditor({
            title: article.title,
            description: article.description,
            body: article.body,
          });
          setTagList(article.tagList);
        } catch (e) {
          errHandler(e);
          navigate("/", { replace: true });
        }
      }

      initArticle().then(() => setLoading(false));
    }
  }, [URLSlug, isLogin, user.username]);

  useEffect(() => {
    if (preview === true) {
      setPreviewHeight(bounds.height);
      setPreviewWidth(bounds.width);
    }
  }, [preview]);

  // ANCHOR render
  if (!isLogin) {
    navigate("/", { replace: true });
    return <></>;
  }

  if (loading) return <p>loading...</p>;
  return (
    <>
      <HelmetProvider>
        <Helmet>
          <title>Editor - Conduit</title>
          <style type="text/css">
            {`
            .editor-preview {
              height: ${previewHeight}px;
              padding-left: .5rem;
              padding-top: .75rem;
              padding-buttom: .75rem;
              padding-right: .5rem;
              overflow-y: scroll;
              border: 1px solid rgba(0,0,0,.15);
              border-radius: .25rem;
              word-break: break-all;
            }
            
            .editor-preview img {
              width: ${previewWidth/2}px;
              margin: auto;
            }
            `}
          </style>
        </Helmet>
      </HelmetProvider>

      <div className="editor-page">
        <div className="container page">
          <div className="row">
            <div className="col-md-10 offset-md-1 col-xs-12">
              <ul className="error-messages">
                {
                  [
                    <></>,
                    <li>{`title ${errMsg}`}</li>,
                    <li>{`description ${errMsg}`}</li>,
                    <li>{`body ${errMsg}`}</li>,
                  ][error.index]
                }
              </ul>

              <form
                onSubmit={async (e) => {
                  e.preventDefault();
                  setPublishLoading(true);
                  await publishArticle();
                  setPublishLoading(false);
                }}
              >
                <fieldset>
                  <fieldset className="form-group">
                    <input
                      type="text"
                      className="form-control"
                      placeholder="title"
                      name="title"
                      value={editor.title}
                      onChange={onChange}
                      disabled={publishLoading}
                    />
                  </fieldset>
                  <fieldset className="form-group">
                    <input
                      type="text"
                      className="form-control"
                      placeholder="What's this article about?"
                      name="description"
                      value={editor.description}
                      onChange={onChange}
                      disabled={publishLoading}
                    />
                  </fieldset>
                  <fieldset className="form-group">
                    {!preview ? (
                      <textarea
                        ref={ref}
                        className="form-control"
                        name="body"
                        rows={20}
                        placeholder="Write your article (in markdown)"
                        value={editor.body}
                        onChange={onChange}
                        disabled={publishLoading}
                      ></textarea>
                    ) : (
                      <ReactMarkdown
                        children={editor.body}
                        remarkPlugins={[remarkGfm]}
                        className="editor-preview"
                      />
                    )}
                  </fieldset>
                  <fieldset className="form-group">
                    <button
                      className="btn btn-sm 
                      pull-xs-right 
                      btn-secondary"
                      onClick={(e) => {
                        e.preventDefault();
                        setPreview((preview) => !preview);
                      }}
                    >
                      <i
                        className={`${
                          preview ? "ion-eye-disabled" : "ion-eye"
                        }`}
                      />
                      {" Preview"}
                    </button>
                  </fieldset>
                  <fieldset className="form-group">
                    <input
                      type="text"
                      className="form-control"
                      placeholder="Entertags"
                      name="tag"
                      value={tag}
                      onChange={onChange}
                      onKeyDown={onCreateTag}
                      disabled={publishLoading}
                    />
                    <div className="tag-list">
                      {tagList.map((tag, inedx) => (
                        <RemovableTag key={inedx} name={tag} />
                      ))}
                    </div>
                  </fieldset>
                  <button
                    className="btn btn-lg pull-xs-right btn-primary"
                    disabled={publishLoading}
                  >
                    Publish Article
                  </button>
                </fieldset>
              </form>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
