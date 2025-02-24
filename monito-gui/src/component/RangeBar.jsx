export const RangeBar = ({ params, step, onChange, className }) => {
  const combinedClassName = `range my-2 col-start-1 col-end-4 ${className} w-full`;
  return (
    <div className='grid grid-cols-4 items-center justify-start w-full'>
      <input
        type='range'
        min={params.min}
        max={params.max}
        value={params.current}
        className={combinedClassName}
        step={step}
        onChange={onChange}
      />
      <p className='font-sans font-medium col-start-4 text-lg ml-3'>
        {params.current}
      </p>
    </div>
  );
};
