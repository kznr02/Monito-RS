import { RangeBar } from "./RangeBar";
import { useMount, useUpdateEffect } from "ahooks";
import { useState } from "react";
import { getMonitorColor, setMonitorColor } from "../utils/cmd";

export const ColorItem = ({ select, step }) => {
  const [redParam, setRedParam] = useState({ current: 0, max: 0, min: 0 });
  const [greenParam, setGreenParam] = useState({ current: 0, max: 0, min: 0 });
  const [blueParam, setBlueParam] = useState({ current: 0, max: 0, min: 0 });

  const onRedChange = (event) => {
    const newValue = Number(event.target.value);
    setRedParam({ ...redParam, current: newValue });
  };

  const onGreenChange = (event) => {
    const newValue = Number(event.target.value);
    setGreenParam({ ...greenParam, current: newValue });
  };

  const onBlueChange = (event) => {
    const newValue = Number(event.target.value);
    setBlueParam({ ...blueParam, current: newValue });
  };

  useMount(async () => {
    const c = await getMonitorColor(select);
    setRedParam(c.r);
    setGreenParam(c.g);
    setBlueParam(c.b);
  });

  useUpdateEffect(() => {
    setMonitorColor(select, "Red", redParam.current);
  }, [redParam]);

  useUpdateEffect(() => {
    setMonitorColor(select, "Green", greenParam.current);
  }, [greenParam]);

  useUpdateEffect(() => {
    setMonitorColor(select, "Blue", blueParam.current);
  }, [blueParam]);

  return (
    <div>
      <RangeBar
        className='range-error'
        alt='range-red'
        params={redParam}
        step={step}
        onChange={onRedChange}
      />

      <RangeBar
        className='range-success'
        alt='range-green'
        params={greenParam}
        step={step}
        onChange={onGreenChange}
      />

      <RangeBar
        className='range-info'
        alt='range-blue'
        params={blueParam}
        step={step}
        onChange={onBlueChange}
      />
    </div>
  );
};
