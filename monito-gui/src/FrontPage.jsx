import { getVersion } from "@tauri-apps/api/app";
import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import { ReactSVG } from "react-svg";
import TitleBorder from "./utils/TitleBorder";
import { Link, useNavigate } from "react-router";
import ScreenSvg from "/src/assets/Screen.svg";
import { LogoIcon } from "./component/LogoIcon";

const MAX_NAME_LEN = 15;

const FrontPage = () => {
  const [version, setVersion] = useState("Loading...");

  const [monitor_list, setMonitorList] = useState([]);

  const navigate = useNavigate();

  const jumpToSecond = (index) => {
    navigate("/setting", {
      state: {
        monitors: monitor_list,
        select: index,
      },
    });
  };

  useEffect(() => {
    const fetchMonitorList = async () => {
      const tmp_list = await invoke("get_monitor_inst_simple_list");
      const list = tmp_list.map((item) => {
        let l = {
          name: item.name,
          path: item.path,
        };

        if (item.name.length > MAX_NAME_LEN) {
          let clean_name = item.name.slice(0, MAX_NAME_LEN);
          l.name = clean_name;
        }

        if (item.path.length > MAX_NAME_LEN) {
          let clean_path = item.name.slice(0, MAX_NAME_LEN);
          l.path = clean_path;
        }

        return l;
      });

      setMonitorList(list);
    };

    fetchMonitorList();

    const fetchVerison = async () => {
      const appVerison = await getVersion();

      setVersion(appVerison);
    };

    fetchVerison();
  }, []);

  return (
    <div className='h-dvh w-dvw'>
      <TitleBorder />

      <div className='fixed flex flex-col items-center justify-center h-dvh w-dvw'>
        <header className='flex-1 flex items-center justify-center'>
          <div className='flex-1 flex items-start w-dvw'>
            <p className='text-3xl font-sans ml-16 select-none'>Monito</p>
          </div>
          <div className='flex-4'></div>
          <div className='flex-1'>
            {/* <div>
              <button className='btn btn-primary'>设置</button>
            </div> */}
          </div>
        </header>
        <div className='flex-4 flex items-center justify-center w-dvw'>
          <div className='flex'>
            {/* {monitor_list.length >= 2 ? (
              <button className='btn flex-col h-30 w-30 mx-2'>
                <ReactSVG
                  src={ScreenSvg}
                  beforeInjection={(svg) => {
                    svg.setAttribute("style", "width: 48px; height: 48px");
                  }}
                />
                <p className='font-normal text-xs'>共同控制</p>
              </button>
            ) : (
              <></>
            )} */}

            {monitor_list.map((m, index) => (
              <button
                key={index}
                className='btn flex-col h-30 w-30 mx-2'
                onClick={() => jumpToSecond(index)}
              >
                <LogoIcon
                  monitor={m}
                  svgIcon={ScreenSvg}
                  width={"48px"}
                  height={"48px"}
                />
              </button>
            ))}
          </div>
        </div>
        <div className='flex-1'>
          <p className='text-gray-300 select-none'>version {version}</p>
        </div>
      </div>
    </div>
  );
};

export default FrontPage;
