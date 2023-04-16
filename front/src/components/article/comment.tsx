import { useSetAtom } from "jotai";
import { Comment as CommentApi } from "../../api/comment";
import { atomComments } from "../../stores/article";
import { Link } from "react-router-dom";

interface CommentProps {
  comment: CommentApi.Comment;
  username: string;
  slug: string;
}

export function Comment({ comment, username, slug }: CommentProps) {
  // ANCHOR store
  const setComments = useSetAtom(atomComments);

  // ANCHOR event
  async function onDeleteComment() {
    await CommentApi.del.handler(slug, comment.id);
    setComments((comments) =>
      comments.filter((innerComment) => innerComment.id !== comment.id)
    );
  }

  return (
    <div className="card">
      <div className="card-block">
        <p className="card-text">{comment.body}</p>
      </div>
      <div className="card-footer">
        <Link
          to={`/profile/${comment.author.username}`}
          className="comment-author"
        >
          <img
            src={comment.author.image}
            alt={comment.author.username}
            className="comment-author-img"
          />
        </Link>{" "}
        <Link
          to={`/profile/${comment.author.username}`}
          className="comment-author"
        >
          {comment.author.username}
        </Link>
        <span className="date-posted">
          {new Date(comment.createdAt).toLocaleDateString("en-US", {
            year: "numeric",
            month: "long",
            day: "numeric",
          })}
        </span>
        {comment.author.username === username ? (
          <span className="mod-options">
            <i className="ion-trash-a" onClick={onDeleteComment} />
          </span>
        ) : (
          <></>
        )}
      </div>
    </div>
  );
}
