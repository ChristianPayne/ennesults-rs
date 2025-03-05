import { z } from "zod";

export const formSchema = z.object({
  autoConnectOnStartup: z.boolean(),
  channelName: z.string().min(2),
  // botName: z.string().min(2).optional(),
  // oauthTokenValue: z.string().min(30).max(30).optional(),

  enableWhispers: z.boolean(),
  usersAllowedToWhisper: z.string(),
  enableAnnouncements: z.boolean(),
  // Add zod validation to make sure the minimum time is less than the maximum time.
  maximumTimeBetweenAnnouncements: z.coerce.number().min(0),
  minimumTimeBetweenAnnouncements: z.coerce.number().min(0),
  randomizeAnnouncements: z.boolean(),
  enableInsults: z.boolean(),
  maximumTimeBetweenInsults: z.coerce.number().min(0),
  minimumTimeBetweenInsults: z.coerce.number().min(0),
  lurkTime: z.coerce.number().positive(),
  enableComebacks: z.boolean(),
  percentChanceOfComeback: z.coerce.number().min(0).max(100),
  comebackExceptions: z.string(),
  enableCorrections: z.boolean(),
  percentChanceOfCorrection: z.coerce.number().min(0).max(100),
  correctionExceptions: z.string(),
  messageQueueInterval: z.coerce.number().min(0),
})
.refine((settings) => {
  if (settings.minimumTimeBetweenInsults > settings.maximumTimeBetweenInsults) {
    return false;
  }
  return true;
}, {
    message:
      "Minimum time between insults must be less than the maximum time between insults.",
  },
)
.refine((settings) => {
  if (settings.minimumTimeBetweenAnnouncements > settings.maximumTimeBetweenAnnouncements) {
    return false;
  }
  return true;
}, {
  message:
    "Minimum time between announcements must be less than the maximum time between announcements.",
});

export type FormSchema = typeof formSchema;