export const getFormValues = (formRef) => {
  const names = [...new Set([...(formRef?.elements || [])].map((el) => el.name).filter(Boolean))];

  const getValueFromElement = (el) => {
    const type = el.type || `${el[0].type}-multiple`;
    switch (type) {
      case "select-multiple":
        if (el.selectedOptions.length) {
          return [...el.selectedOptions].map((option) => JSON.parse(option?.value || ""));
        } else if (el.options.length) {
          return [...el.options].map((option) => JSON.parse(option?.value || ""));
        }
        return [];
      case "checkbox":
        return !!el.checked;
      case "checkbox-multiple":
        return [...el].filter((el) => el.checked).map((el) => [...el.labels].reverse().find((label) => !!label.innerText)?.innerText);
      case "radio-multiple":
        return [...([...el].find((el) => el.checked)?.labels || [])].reverse().find((label) => !!label.innerText)?.innerText;
      case "file":
        return el.files[0];
      default:
        return el.value;
    }
  };

  let values = {};
  names.forEach((name) => {
    const el = formRef.elements[name];
    const value = getValueFromElement(el);
    if (value === null || value === undefined) return;
    values[name] = value;
  });

  return values;
};

export const getName = (el = {}) => el.name || el.attributes?.name?.value;

export const getNamedValues = (containerRef) => {
  const namedElements = [...new Set([...(containerRef?.getElementsByTagName("*") || [])].filter((el) => getName(el)).filter(Boolean))];

  const getValueFromElement = (el) => {
    const type = el?.type || `${el?.[0]?.type}-multiple`;
    switch (type) {
      case "select-multiple":
        if (el.selectedOptions.length) {
          return [...el.selectedOptions].map((option) => JSON.parse(option?.value || ""));
        } else if (el.options.length) {
          return [...el.options].map((option) => JSON.parse(option?.value || ""));
        }
        return [];
      case "checkbox":
        return !!el.checked;
      case "checkbox-multiple":
        return [...el].filter((el) => el.checked).map((el) => [...el.labels].reverse().find((label) => !!label.innerText)?.innerText);
      case "radio-multiple":
        return [...([...el].find((el) => el.checked)?.labels || [])].reverse().find((label) => !!label.innerText)?.innerText;
      case "file":
        return el.files[0];
      default:
        return el.value || el.attributes?.value?.value;
    }
  };

  let values = {};
  namedElements.forEach((el) => {
    const value = getValueFromElement(el);
    if (value === null || value === undefined) return;
    values[getName(el)] = value;
  });

  return values;
};
