import { Secret } from '../dtos/secret';
import jwt from "jsonwebtoken";

export class JWT {
  public static decrypt_token(token: string): any{
    const result = jwt.verify(token, "secret");

    return result;

  }
}