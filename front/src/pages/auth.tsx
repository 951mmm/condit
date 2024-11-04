import { ChangeEvent, FormEvent, useEffect, useState } from "react";
import { Helmet, HelmetProvider } from "react-helmet-async";
import { Link, useNavigate } from "react-router-dom";
import { User } from "../api/user";
import { useAtom } from "jotai";
import { atomIsLogin, atomUser } from "../stores/auth";
import { LoginError } from "../components/auth/login-error";
import { RegisterError } from "../components/auth/register-error";
import { LoginForm } from "../components/auth/login-form";
import { RegisterForm } from "../components/auth/register-form";

interface AuthProps {
  type: "login" | "register";
}
export function Auth({ type }: AuthProps) {
  return (
    <>
      <HelmetProvider>
        <Helmet>
          <title>Sign{` ${type === "login" ? "in" : "up"} `}- Conduit</title>
        </Helmet>
      </HelmetProvider>

      <div className="auth-page">
        <div className="container page">
          <div className="row">
            <div className="col-md-6 offset-md-3 col-xs-12">
              <h1 className="text-xs-center">
                Sign{` ${type === "login" ? "in" : "up"}`}
              </h1>
              <p className="text-xs-center">
                {type === "login" ? (
                  <Link to="/register" className="text-xs-center">
                    Need an account?
                  </Link>
                ) : (
                  <Link to="/login" className="text-xs-center">
                    Had an account?
                  </Link>
                )}
              </p>

              {type === "login" ? <LoginError /> : <RegisterError />}
              {type === "login" ? <LoginForm /> : <RegisterForm />}
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
