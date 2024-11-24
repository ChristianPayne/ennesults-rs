import { toast } from "svelte-sonner";
import type { Alert } from "../../types";
import { nanoid } from 'nanoid';
import { writable, get, type Writable } from 'svelte/store';

export const notifications$: Writable<Notification[]> = writable([]);

export type Notification = {
  type: Alert,
  id: string,
  title: string,
  seen: boolean,
  timestamp: number,
  description?: string,
}

export type NotificationBase = Omit<Notification, "id" | "seen" | "type" | "timestamp">

export function addNotification(type: Alert, notif: NotificationBase) {
  let notification: Notification = {
    type,
    id: nanoid(),
    seen: false,
    timestamp: Date.now(),
    ...notif
  }
  notifications$.update(existing => [...existing, notification])
}


export function markNotificationAsSeen(id: string) {
  let found = get(notifications$).find(n => n.id === id);
  if (found) {
    found.seen = true
  }
  notifications$.update(existing => existing)
}

export function markAllAsSeen() {
  let updated = get(notifications$).map(n => {
    return {
      ...n,
      seen: true
    }
  })
  notifications$.set(updated);
}

export function alertNotification(type: Alert, notification: NotificationBase) {
  switch (type) {
    case "System": {
      toast(notification.title, {
        ...(notification?.description && { description: notification.description })
      })
      // addNotification(type, notification);
      break;
    }
    case "Info": {
      toast.success(notification.title, {
        ...(notification?.description && { description: notification.description })
      })
      addNotification(type, notification);
      break;
    }
    case "Error": {
      toast.error(notification.title, {
        ...(notification?.description && { description: notification.description })
      })
      addNotification(type, notification);
      break;
    }
    case "Warn": {
      toast.warning(notification.title, {
        ...(notification?.description && { description: notification.description })
      })
      addNotification(type, notification);
      break;
    }
  }
}