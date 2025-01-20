export const openTab = (id: string, button?: Element | null) => {
  const tab = document.getElementById(`tab-${id}`);
  if (!tab) return;

  Object.values(document.getElementsByClassName('tab-active')).map(_tab => _tab.classList.remove('tab-active'));
  Object.values(document.getElementsByClassName('tab-button-active')).map(_tab => _tab.classList.remove('tab-button-active'));
  tab.classList.add('tab-active');

  if (button) {
    button.classList.add('tab-button-active');
  }
}