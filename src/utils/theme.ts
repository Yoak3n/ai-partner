import type{ GlobalThemeOverrides} from 'naive-ui'
export const themeOverrides: GlobalThemeOverrides = {
  Menu: {
    borderColorHorizontal: '#0000',
    itemHeight: '42px',
    borderRadius: '3px',
    fontSize: '14px',
    dividerColor: 'rgb(239, 239, 245)',
    groupTextColor: 'rgb(118, 124, 130)',
    itemTextColor: '#666666',
    itemTextColorHover: '#4a90e2',
    itemTextColorActive: '#4a90e2',
    itemTextColorChildActive: '#4a90e2',
    itemTextColorChildActiveHover: '#4a90e2',
    itemTextColorActiveHover: '#4a90e2',
    itemIconColor: '#888888',
    itemIconColorHover: '#4a90e2',
    itemIconColorActive: '#4a90e2',
    itemIconColorActiveHover: '#4a90e2',
    itemIconColorChildActive: '#4a90e2',
    itemIconColorChildActiveHover: '#4a90e2',
    itemIconColorCollapsed: '#666666',
    itemTextColorHorizontal: '#666666',
    itemTextColorHoverHorizontal: '#4a90e2',
    itemTextColorActiveHorizontal: '#4a90e2',
    itemTextColorChildActiveHorizontal: '#4a90e2',
    itemTextColorChildActiveHoverHorizontal: '#4a90e2',
    itemTextColorActiveHoverHorizontal: '#4a90e2',
    itemIconColorHorizontal: '#666666',
    itemIconColorHoverHorizontal: '#4a90e2',
    itemIconColorActiveHorizontal: '#4a90e2',
    itemIconColorActiveHoverHorizontal: '#4a90e2',
    itemIconColorChildActiveHorizontal: '#4a90e2',
    itemIconColorChildActiveHoverHorizontal: '#4a90e2',
    arrowColor: '#666666',
    arrowColorHover: '#4a90e2',
    arrowColorActive: '#4a90e2',
    arrowColorActiveHover: '#4a90e2',
    arrowColorChildActive: '#4a90e2',
    arrowColorChildActiveHover: '#4a90e2',
    itemColorHover: 'rgba(74, 144, 226, 0.06)',
    itemColorActive: 'rgba(74, 144, 226, 0.1)',
    itemColorActiveHover: 'rgba(74, 144, 226, 0.15)',
    itemColorActiveCollapsed: 'rgba(74, 144, 226, 0.1)',
  },
  Popconfirm: {
    contentTextColor: '#666666',
    iconColor: '#4a90e2',
    borderRadius: '8px',
    padding: '16px',
    color: '#ffffff',
    boxShadow: '0 2px 12px rgba(74, 144, 226, 0.12)',
    
    // 使用与控制台显示的 CSS 变量对应的属性名
    // 确认按钮样式
    actionButtonColor: '#4a90e2',           // 对应 --n-color
    actionButtonColorHover: '#5c9de6',      // 对应 --n-color-hover
    actionButtonColorPressed: '#3d7abd',    // 对应 --n-color-pressed
    actionButtonTextColor: '#ffffff',       // 对应 --n-text-color
    actionButtonTextColorHover: '#ffffff',  // 对应 --n-text-color-hover
    actionButtonTextColorPressed: '#ffffff',// 对应 --n-text-color-pressed
    actionButtonBorderColor: '#4a90e2',     // 对应 --n-border
    actionButtonBorderHoverColor: '#5c9de6',// 对应 --n-border-hover
    actionButtonBorderPressedColor: '#3d7abd', // 对应 --n-border-pressed
    
    // 其他样式属性
    fontSize: '14px',
    titleFontSize: '15px',
    titleFontWeight: '600',
    titleTextColor: '#333333',
    
    // 取消按钮样式可能需要单独设置
    negativeButtonColor: '#f5f7fa',
    negativeButtonTextColor: '#666666',
    negativeButtonBorderColor: '#e0e0e6',
    negativeButtonColorHover: '#f0f2f5',
    negativeButtonTextColorHover: '#4a90e2',
    negativeButtonColorPressed: '#e8eaed',
    negativeButtonTextColorPressed: '#3d7abd',
  },
  Button: {
    textColor: '#666666',
    textColorHover: '#4a90e2',
    textColorPressed: '#3d7abd',
    textColorFocus: '#4a90e2',
  }
}

export const modalThemeOverrides: GlobalThemeOverrides = {
  Input: {
    borderHover: '#4a90e2',
    borderFocus: '#4a90e2',
    boxShadowFocus: '0 0 0 2px rgba(74, 144, 226, 0.2)',
    loadingColor: '#4a90e2',
    color: '#fff',
    colorFocus: '#fff',
    textColor: '#666666',
    placeholderColor: '#999',
  },
  Select: {
    peers: {
      InternalSelection: {
        textColor: '#666666',
        border: '1px solid rgb(224, 224, 230)',
        borderHover: '#4a90e2',
        borderFocus: '#4a90e2',
        borderActive: '#4a90e2',
        borderRadius: '3px',
        boxShadowFocus: '0 0 0 2px rgba(74, 144, 226, 0.2)',
        boxShadowActive: '0 0 0 2px rgba(74, 144, 226, 0.2)',
        caretColor: '#4a90e2',
        color: 'rgba(255, 255, 255, 1)',
        colorActive: 'rgba(255, 255, 255, 1)',
        colorDisabled: 'rgb(250, 250, 252)',
        paddingSingle: '0 26px 0 12px',
        paddingMultiple: '3px 26px 0 12px',
        placeholderColor: 'rgba(194, 194, 194, 1)',
        textColorDisabled: 'rgba(194, 194, 194, 1)',
        arrowColor: 'rgba(194, 194, 194, 1)',
        clearColor: 'rgba(194, 194, 194, 1)',
        clearColorHover: 'rgba(146, 146, 146, 1)',
        clearColorPressed: 'rgba(175, 175, 175, 1)',
        clearSize: '16px',
        arrowSize: '16px'
      },
      InternalSelectMenu: {
        optionTextColor: '#666666',
        optionTextColorActive: '#4a90e2',
        optionTextColorPressed: '#3d7abd',
        optionCheckColor: '#4a90e2',
        color: '#fff',
        optionColorPending: 'rgba(74, 144, 226, 0.06)',
        optionColorActive: 'rgba(74, 144, 226, 0.1)',
      }
    }
  },
  Switch: {
    railColorActive: '#4a90e2',
    boxShadowFocus: '0 0 0 2px rgba(74, 144, 226, 0.2)',
    loadingColor: '#4a90e2'
  },
  Button: {
    textColor: '#666666',
    textColorPrimary: '#ffffff',
    textColorHoverPrimary: '#ffffff',  // 改为纯白色
    textColorPressedPrimary: '#ffffff',
    textColorFocusPrimary: '#ffffff',
    borderRadius: '20px',
    heightMedium: '40px',
    fontSizeMedium: '15px',
    fontWeight: '600',  // 加粗字体
    colorPrimary: '#4a90e2',
    colorHoverPrimary: '#5c9de6',  // 调亮一点
    colorPressedPrimary: '#3d7abd',
    colorFocusPrimary: '#4a90e2',
    rippleColor: 'rgba(255, 255, 255, .35)',
    borderPrimary: '1px solid #4a90e2',
    borderHoverPrimary: '1px solid #5c9de6',
    borderPressedPrimary: '1px solid #3d7abd',
    borderFocusPrimary: '1px solid #4a90e2',
  }
}