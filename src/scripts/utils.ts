import twemoji from "twemoji";

export const DiscordInviteLink = "https://discord.gg/whNdXjKezx";
export const getJSON = async (url: string) => await fetch(url).then(async (res) => await res.json());
export const parseEmoji = (text: string | HTMLElement): string | HTMLElement => {
  return twemoji.parse(text, { base: 'https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/' })
}

export async function postJSON(url: string, json: object, successFunc: (...res: any) => any, errorFunc: (...res: any) => any): Promise<[boolean, any]> {
  var success = false
  try {
      const response = await fetch(url, {
          headers: {
              "Content-Type": "application/json",
          },
          method: 'POST',
          body: JSON.stringify(json)
      });

      if (!response.ok) {
          errorFunc(response)
          throw new Error(`Error status: ${response.status}`);
      }
      success = true

      const res = await response;
      if (successFunc != null) {
          return [success, await successFunc(res)]
      } else {
          return [success, res];
      }
  } catch (error) {
      console.error(error);
      return [success, error];
  }
}