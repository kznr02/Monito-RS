export const Collapse = ({ title, className, children }) => {
  const combinedClassName = `${className} mx-2`;
  return (
    <div className={combinedClassName}>
      <div className='collapse collapse-arrow bg-base-200 border border-base-300 my-2'>
        <input type='checkbox' />
        <div className='collapse-title font-semibold'>{title}</div>
        <div className='collapse-content text-sm'>{children}</div>
      </div>
    </div>
  );
};
