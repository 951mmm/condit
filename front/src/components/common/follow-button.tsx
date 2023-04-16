import { useAtom, useAtomValue } from "jotai";
import { atomFollow } from "../../stores/subscribe";
import { Profile } from "../../api/profile";
import { errHandler } from "../../utils";
import { MouseEvent, useState } from "react";
import { atomIsLogin } from "../../stores/auth";
import { useNavigate } from "react-router-dom";

interface FollowButtonProps {
  following: boolean;
  userId: string;
}

export function FollowButton({
  following: propFollowing,
  userId,
}: FollowButtonProps) {
  // ANCHOR state
  const [following, setFollowing] = useState(propFollowing);
  const [loading, setLoading] = useState(false);
  // ANCHOR store
  const isLogin = useAtomValue(atomIsLogin);
  const navigate = useNavigate();

  // ANCHOR event
  async function follow() {
    try {
      await Profile.follow.handler(userId!);
      setFollowing(true);
    } catch (e) {
      errHandler(e);
    }
  }

  async function disFollow() {
    try {
      await Profile.disFollow.handler(userId);
      setFollowing(false);
    } catch (e) {
      errHandler(e);
    }
  }

  async function onClick() {
    if (!isLogin) {
      navigate("/login");
      return;
    }
    setLoading(true);
    following ? await disFollow() : await follow();
    setLoading(false);
  }

  return (
    <button
      type="button"
      className={
        following
          ? `btn btn-sm action-btn btn-secondary ${loading ? "disabled" : ""}`
          : `btn btn-sm action-btn btn-outline-secondary ${
              loading ? "disabled" : ""
            }`
      }
      onClick={onClick}
    >
      <i className="ion-plus-round" />
      {` ${following ? "Unfollow" : "follow"} ${userId}`}
    </button>
  );
}
