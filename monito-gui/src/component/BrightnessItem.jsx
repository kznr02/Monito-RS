import { useState } from "react";
import { RangeBar } from "./RangeBar";
import { useMount } from "ahooks";
import { useUpdateEffect } from "ahooks";
import { updateBrightness } from "../utils/cmd";
import { useDebounceFn } from "ahooks";
import { useAsyncEffect } from "ahooks";
import { getMonitorBrightness } from "../utils/cmd";

export const BrightnessItem = ({ select, step }) => {
  const [brightness, setBrightness] = useState({ current: 0, max: 0, min: 0 });

  const slideChange = (event) => {
    const newValue = Number(event.target.value);
    setBrightness({ ...brightness, current: newValue });
  };

  useAsyncEffect(async () => {
    const fetchBrightness = async () => {
      let b = await getMonitorBrightness(select);
      setBrightness(b);
    };

    await fetchBrightness();
  }, []);

  const { run } = useDebounceFn(updateBrightness, {
    wait: 15,
    leading: true,
    trailing: false,
  });

  useUpdateEffect(() => {
    run(select, brightness.current);
  }, [brightness]);

  return (
    <div className='grid grid-rows-2'>
      <RangeBar
        params={brightness}
        step={step}
        onChange={slideChange}
        className='range-primary'
      />
      <div className='flex items-center'>
        <input type='checkbox' className='checkbox' />
        <p className='text-nowrap mx-2 font-sans font-medium'>Control</p>
      </div>
    </div>
  );
};
