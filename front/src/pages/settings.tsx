import { useAtom } from "jotai";
import { ChangeEvent, FormEvent, useEffect, useReducer, useState } from "react";
import { Helmet, HelmetProvider } from "react-helmet-async";
import { atomIsLogin, atomUser } from "../stores/auth";
import { useNavigate } from "react-router-dom";
import { User } from "../api/user";
import { errHandler } from "../utils";

interface FormStates {
  image: string;
  username: string;
  bio: string;
  email: string;
  password: string;
}

export function Settings() {
  //  ANCHOR state
  const [form, setForm] = useState({
    image: "",
    username: "",
    bio: "",
    email: "",
    password: "",
  });

  const [loading, setLoading] = useState(false);

  // ANCHOR store
  const [isLogin, setIsLogin] = useAtom(atomIsLogin);
  const [user, setUser] = useAtom(atomUser);
  const navigate = useNavigate();

  function onChange(e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) {
    const { name, value } = e.target;
    setForm(form => ({
      ...form,
      [name]: value,
    }));
  }

  function onLogout() {
    localStorage.removeItem("jwtToken");
    setIsLogin(false);
    setUser({
      email: "",
      username: "",
      bio: "",
      image: "",
    });
    navigate("/", { replace: true });
  }

  async function onUpdateProfile(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    setLoading(true);
    try {
      const { user } = await User.put.handler({
        user: form,
      });

      const { token, ...rest } = user;
      localStorage.setItem("jwtToken", user.token);
      setUser(rest);
    } catch (e) {
      errHandler(e);
    }
    setLoading(false);
  }

  // ANCHOR effect
  useEffect(() => {
    function initSettings() {
      setForm({
        ...user,
        bio: user.bio || "",
        password: "",
      });
    }
    initSettings();
  }, [user]);

  if(!isLogin) navigate("/", {replace: true});

  return (
    <>
      <HelmetProvider>
        <Helmet>
          <title>Settings — Conduit</title>
        </Helmet>
      </HelmetProvider>

      <div className="setting-page">
        <div className="container page">
          <div className="row">
            <div className="col-md-6 offset-md-3 col-xs-12">
              <h1 className="text-xs-center">Your Settings</h1>
              <form onSubmit={onUpdateProfile}>
                <fieldset>
                  <fieldset className="form-group">
                    <input
                      type="text"
                      placeholder="URL of profile picture"
                      name="image"
                      value={form.image}
                      onChange={onChange}
                      disabled={loading}
                      className="form-control"
                    />
                  </fieldset>
                  <fieldset className="form-group">
                    <input
                      type="text"
                      className="form-control form-control-lg"
                      placeholder="Your name"
                      name="username"
                      value={form.username}
                      onChange={onChange}
                      disabled={loading}
                      autoComplete="off"
                    />
                  </fieldset>
                  <fieldset className="form-group">
                    <textarea
                      name="bio"
                      rows={8}
                      placeholder="Short bio about you"
                      value={form.bio}
                      onChange={onChange}
                      disabled={loading}
                      className="form-control form-control-lg"
                    />
                  </fieldset>
                  <fieldset className="form-group">
                    <input
                      type="email"
                      placeholder="email"
                      name="email"
                      value={form.email}
                      onChange={onChange}
                      disabled={loading}
                      autoComplete="off"
                      className="form-control form-control-lg"
                    />
                  </fieldset>
                  <fieldset className="form-group">
                    <input
                      className="form-control form-control-lg"
                      type="password"
                      placeholder="New Password"
                      name="password"
                      value={form.password}
                      onChange={onChange}
                      disabled={loading}
                    />
                  </fieldset>
                  <button
                    type="submit"
                    className="btn btn-lg btn-primary pull-xs-right"
                  >
                    Update Settings
                  </button>
                </fieldset>
              </form>
              <hr />
              <button
                type="button"
                className="btn btn-outline-danger"
                onClick={onLogout}
              >
                Or click here to logout
              </button>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
