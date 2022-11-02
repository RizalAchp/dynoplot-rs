#pragma once
#ifndef __DYNO_PLOT_HEADER__
#define __DYNO_PLOT_HEADER__

typedef int ReturnCODE_;

enum ReturnCODE : ReturnCODE_ {
  PLOTSUCCESS = 0,
  PLOTERROR = 1,
};

#ifdef __cplusplus
extern "C" {
#else
extern {
#endif

/// initilize plot dyno for multi plot
///
/// @param _path path to save the plot image
/// @param _name name to display in plot image
/// @param _xmin min value of item in array of x axis data
/// @param _xmax max value of item in array of x axis data
/// @param _ymin min value of item in array of y axis data
/// @param _ymax max value of item in array of y axis data
ReturnCODE_ DYNOPLOT_init_config(const char *_path, const char *_name,
                                 const float _xmin, const float _xmax,
                                 const float _ymin, const float _ymax);

/// create and save to path given in param
///
/// @param _x pointer to array of x axis data
/// @param _y pointer to array of y axis data
/// @param _size size of all item in array of y axis data
ReturnCODE_ DYNOPLOT_save_plot(const float *_x, const float *_y,
                               const int _size);

ReturnCODE_ DYNOPLOT_SaveMultiPlot(const float *, const float *, const float *,
                                   const float *, const float *, const int);
}
#endif
