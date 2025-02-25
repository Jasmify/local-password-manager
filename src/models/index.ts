import { z } from "zod";

export const formSchema = z.object({
  accountName: z.string().min(1, "This field is required"),
  identifier: z.string().min(1, "This field is required"),
  passwords: z
    .array(z.string().min(1, "Password cannot be empty"))
    .min(1, "At least one password is required"),
  categoryName: z.string().min(1, "This field is required"),
});

export type FormData = z.infer<typeof formSchema>;

export interface AccountSummary {
  accountUlid: string;
  accountName: string;
  identifierUlid: string;
  identifier: string;
  categoryName: string;
}

export interface PasswordInfo {
  id: number;
  passwordRaw: string;
}

export interface SearchCriteria {
  accountName: string;
  identifier: string;
  categoryName: string;
}

export interface AccountInfo {
  accountUlid: string;
  accountName: string;
  identifierUlid: string;
  identifier: string;
  passwords: PasswordInfo[];
  categoryName: string;
}
