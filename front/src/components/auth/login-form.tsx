import { ChangeEvent, FormEvent, useEffect, useState } from "react";
import { User } from "../../api/user";
import { useAtom, useSetAtom } from "jotai";
import { atomError, atomIsLogin, atomUser } from "../../stores/auth";
import { useNavigate } from "react-router-dom";

export function LoginForm() {
  // ANCHOR STATE
  const [loading, setLoading] = useState(false);
  const [form, setForm] = useState({
    email: "",
    password: "",
  });

  // ANCHOR store
  const [isLogin, setIsLogin] = useAtom(atomIsLogin);
  const setUser = useSetAtom(atomUser);
  const setError = useSetAtom(atomError);
  const navigate = useNavigate();

  // ANCHOR event
  function onChange(e: ChangeEvent<HTMLInputElement>) {
    const { name, value } = e.target;
    setForm((form) => ({
      ...form,
      [name]: value,
    }));
  }

  async function onLogin(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    setLoading(true);
    try {
      const data = await User.login.handler({
        user: form,
      });
      const { token, ...rest } = data.user;
      localStorage.setItem("jwtToken", token);
      setIsLogin(true);
      setUser(rest);
      navigate("/", { replace: true });
    } catch (e: any) {
      const errMsg = e.response.data.errors;

      errMsg.email &&
        setError({
          index: 1,
          msgs: errMsg.email,
        });
      errMsg.password &&
        setError({
          index: 2,
          msgs: errMsg.password,
        });
      errMsg["email or password"] &&
        setError({
          index: 3,
          msgs: errMsg["email or password"],
        });
    }
    setLoading(false);
  }

  // ANCHOR effect
  useEffect(() => {
    if (isLogin) navigate("/", { replace: true });
  }, [isLogin]);
  return (
    <form onSubmit={onLogin}>
      <fieldset className="form-group">
        <input
          className="form-control form-control-lg"
          placeholder="email"
          type="email"
          name="email"
          value={form.email}
          onChange={onChange}
          disabled={loading}
          autoComplete="off"
        />
      </fieldset>
      <fieldset className="form-group">
        <input
          className="form-control form-control-lg"
          placeholder="password"
          type="password"
          name="password"
          value={form.password}
          onChange={onChange}
          disabled={loading}
        />
      </fieldset>
      <button
        className="btn btn-lg btn-primary pull-xs-right"
        disabled={loading}
      >
        Sign in
      </button>
    </form>
  );
}
