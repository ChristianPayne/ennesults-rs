import { z } from "zod";
 
export const formSchema = z.object({
  autoConnectOnStartup: z.boolean(),
  channelName: z.string().min(2),
  botName: z.string().min(2).optional(),
  oauthTokenValue: z.string().min(30).max(30).optional(),
  enableWhispers: z.boolean(),
  usersAllowedToWhisper: z.string(),
  enableInsults: z.boolean(),
  minimumUsersInChatToInsult: z.coerce.number().positive(),
  enableComebacks: z.boolean(),
  percentChanceOfComeback: z.coerce.number().min(0).max(100),
  comebackExceptions: z.string(),
  enableCorrections: z.boolean(),
  percentChanceOfCorrection: z.coerce.number().min(0).max(100),
  correctionExceptions: z.string(),
});
 
export type FormSchema = typeof formSchema;