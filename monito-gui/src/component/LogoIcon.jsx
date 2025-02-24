import { ReactSVG } from "react-svg";

export const LogoIcon = ( {monitor, svgIcon, height, width, className} ) => {
    return (
      <div className='flex flex-col items-center justify-center'>
        <ReactSVG
          src={svgIcon}
          beforeInjection={(svg) => {
            svg.setAttribute("style", `height: ${height}; width: ${width};`);
          }}
        />
        <p className='font-normal text-xs'>{monitor.name}</p>
        <p className='font-medium text-xs'>{monitor.path}</p>
      </div>
    );
}