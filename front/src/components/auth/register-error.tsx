import { useAtom, useAtomValue } from "jotai";
import { atomError } from "../../stores/auth";

export function RegisterError() {
  const { index, msgs } = useAtomValue(atomError);
  const msg = msgs.join(" ");
  return (
    <ul className="error-messages">
      {
        [
          <></>,
          <li>email {msg}</li>,
          <li>username {msg}</li>,
          <li>password {msg}</li>,
        ][index]
      }
    </ul>
  );
}
