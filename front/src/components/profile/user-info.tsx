import { useAtom } from "jotai";
import { useEffect, useState } from "react";
import { atomUser } from "../../stores/auth";
import { Link, useNavigate, useParams } from "react-router-dom";
import { Profile } from "../../api/profile";
import { errHandler } from "../../utils";
import { atomFollow } from "../../stores/subscribe";
import { FollowButton } from "../common/follow-button";
import { atomSetFeedQuery } from "../../stores/feed";

interface UserInfoProps {
  userId: string;
}
export function UserInfo({ userId }: UserInfoProps) {
  // ANCHOR state
  const [userInfo, setUserInfo] = useState({
    // local
    image: "",
    username: "",
    bio: "",
    // down-stream
    following: false,
  });
  const [loading, setLoading] = useState(false);

  // ANCHOR store
  const [user] = useAtom(atomUser);
  const navigate = useNavigate();

  // ANCHOR effect
  useEffect(() => {
    async function initUserInfo() {
      setLoading(true);
      try {
        const data = await Profile.get.handler(userId!);
        setUserInfo(data.profile);
      } catch (e) {
        navigate("/", { replace: true });
      }
      setLoading(false);
    }

    initUserInfo();
  }, [userId]);

  return (
    <div className="user-info">
      {loading ? (
        <>loading...</>
      ) : (
        <div className="container">
          <div className="row">
            <div className="col-xs-12 col-md-10 offset-md-1">
              <img src={userInfo.image} alt="profile" className="user-img" />
              <h4>{userInfo.username}</h4>
              <p>{userInfo.bio}</p>
              {user.username === userId ? (
                <Link
                  to="/settings"
                  className="btn btn-sm btn-outline-secondary action-btn"
                >
                  <i className="ion-gear-a" />
                  Edit Profile Settings
                </Link>
              ) : (
                <FollowButton following={userInfo.following} userId={userId!} />
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
