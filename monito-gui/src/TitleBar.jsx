import { Window } from "@tauri-apps/api/window";
import { useEffect } from "react";
import { ReactSVG } from "react-svg";

const BasicWindow = ({ children }) => {
  const appWindow = new Window("main");

  useEffect(() => {
    document
      .getElementById("titlebar-minimize")
      ?.addEventListener("click", () => appWindow.minimize());
    document
      .getElementById("titlebar-maximize")
      ?.addEventListener("click", () => appWindow.toggleMaximize());
    document
      .getElementById("titlebar-close")
      ?.addEventListener("click", () => appWindow.close());
  }, []);

  return (
    <div className='overflow-hidden'>
      <div data-tauri-drag-region className='titlebar'>
        <button className='titlebar-button' id='titlebar-minimize'>
          <ReactSVG
            src='https://api.iconify.design/mdi:window-minimize.svg'
            alt='minimize'
          />
        </button>
        <button className='titlebar-button' id='titlebar-maximize'>
          <ReactSVG
            src='https://api.iconify.design/mdi:window-maximize.svg'
            alt='maximize'
          />
        </button>
        <button className='titlebar-button' id='titlebar-close'>
          <ReactSVG
            src='https://api.iconify.design/mdi:close.svg'
            alt='close'
          />
        </button>
      </div>

      {children}
    </div>
  );
};

export default BasicWindow;
