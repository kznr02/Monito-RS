import ScreenSvg from "/src/assets/Screen.svg";
import { LogoIcon } from "./component/LogoIcon";
import { BrightnessItem } from "./component/BrightnessItem";
import { Collapse } from "./component/Collapse";
import { ColorItem } from "./component/ColorItem";

const ConfigPage = ({ item, select }) => {
  return (
    <div className='flex items-center justify-center h-full'>
      <div className='flex flex-col flex-2 items-center justify-center h-full w-full'>
        <div className='flex flex-1 items-end'>
          <LogoIcon
            monitor={item}
            svgIcon={ScreenSvg}
            height={"60px"}
            width={"60px"}
          />
        </div>

        <div className='flex-1'>
          {/* <button className='btn btn-primary'>标识</button> */}
        </div>
      </div>

      <div className='flex-0 divider divider-horizontal'></div>

      <div className='flex-6 w-full h-full overflow-y-auto min-[700px]:flex min-[700px]:flex-wrap '>
        <div className='min-[700px]:w-1/2'>
          <Collapse title={`Brightness`} className='min-[700px]:'>
            <BrightnessItem select={select} step={1} />
          </Collapse>
        </div>

        <div className='min-[700px]:w-1/2'>
          <Collapse title={`Color`} className='min-[700px]:'>
            <ColorItem select={select} step={5} />
          </Collapse>
        </div>
      </div>
    </div>
  );
};

export default ConfigPage;
