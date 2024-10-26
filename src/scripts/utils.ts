import twemoji from "twemoji";

export async function getJSON(url: string, func: (...json: any) => any ): Promise<[boolean, any]> {
  let success: boolean = false
  try {
      const response = await fetch(url);
      if (!response.ok) {
          throw new Error(`Response status: ${response.status}`);
      }
      success = true

      const json = await response.json();
      if (func != null) {
        return [success, await func(json)]
      } else {
          return [success, json];
      }
  } catch (error: any) {
      console.error(error.message);
      return [success, error];
  }
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

export function parseEmoji(text: string | HTMLElement): string | HTMLElement {
  return twemoji.parse(text, { base: 'https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/' })
}

export function hideAllPopups() {
  const popups = document.querySelectorAll('[data-popup-visible="true"]')
  popups.forEach((popup) => {
    popup.setAttribute("data-popup-visible", "false")
  })
}

export function openPopup(popupName: string) {
  hideAllPopups()
  const popup = document.querySelector("#popup_"+popupName)
  if (popup) {
    popup.setAttribute("data-popup-visible", "true")
  } else {
    console.error("Can't find popup " + popupName)
  }
}