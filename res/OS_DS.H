/*********************************************************/
/*** OS_SYS.C   V1.0  Written: xx/xx/2017 By:          ***/
/***===================================================***/
/***  Last Updated:           By:                      ***/
/***  Modification:                                    ***/
/***                                                   ***/
/***===================================================***/
/*** Program simulates a simple Operating System.      ***/
/*** Input Files:                                      ***/
/***   1) O/S Parameter File                           ***/
/***   2) Input Queue Data                             ***/
/*** Output Files:                                     ***/
/***   1) Print O/S Start-End times and Parameter data.***/
/***   2) Print Process Allocation... De-allocation    ***/
/***       information.                                ***/
/***   3) If Print_Every_N_Units is not 0, print       ***/
/***       detail information each N units.            ***/
/*********************************************************/

#include "stdafx.h"
#include <io.h>
#include <fcntl.h>
#include <stdio.h>
#include <conio.h>
#include <dos.h>
#include <time.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include <sys\stat.h>
#include <cstdlib>

/*** Defines ***/

#define PGMR "<your name>"
#define VERSION   5                    /* Version for Problem #         */

#define YES "Y"
#define NO  "N"
#define HELP "/?"
#define MAX_ARRAY 33                   /* Maximum array items Start at 1*/
#define MAX_SIZE  32                   /*  Use 0th item for sorting     */
#define MAX_MEM_ARRAY 321              /* Maximum Memory Blks           */
#define MAX_MEM_SIZE  320              /*  Start at 1.                  */
#define RUNAWAY   3000                 /* Must be a runaway OS, kill it */

/* Define O/S process ids (also equivalent to their priorities) */
#define IDLE      0                    /* CPU has nothing to execute    */
#define MASTER   -1                    /* Lowest priority               */
#define DONES    -2
#define IOS      -3
#define DISPATCH -4
#define TI_INT   -5
#define SC_INT   -6
#define IO_INT   -7                    /* Highest priority              */

#define CPU_MASTER    4
#define CPU_DONES     3
#define CPU_IOS       3

#define IO_WORKING   "W"
#define IO_PENDING   "P"
#define IO_DONE      "D"

#define DEV_AVAL     "A"
#define DEV_BUSY     "B"
#define DEV_DISK      1
#define DEV_TAPE      2
#define DEV_CD        3
#define DEV_ARRAY     4

#define READY        "R"
#define EXEC         "E"
#define BLOCKED      "B"
#define DONE         "D"
#define HELD         "H"
#define NONE         " "

#define MEM_FREE      0
#define MEM_USED      1

#define MEM_MODEL_NONE      0
#define MEM_MODEL_FIXED     1
#define MEM_MODEL_VAR       2

/***
 *** File definitions
 ***/

  FILE *osparm_ptr;                    /* osparm_file pointer           */
  char osparm_file[61];                /* Name of the O/S Parm file     */

  FILE *input_ptr;                     /* input_file pointer            */
  char input_file[61];                 /* Name of the input data file   */


/***
 *** Record definitions
 ***/

  struct os_parm_record
    { int  mem_model;                  /* 0=None, 1=Fixed, 2=Variable   */
      int  mem_fix_blksize;            /* F: Number of bytes per block  */
      int  mem_fix_blksaval;           /* F: Number of available blocks */
      int  mem_var_maxsize;            /* V: Maximum block size         */
      int  mem_var_totsize;            /* V: Total available memory     */
      int  pro_max_tasks;              /* Maximum Processes allowed     */
      int  init_quantum;               /* Default quantum size          */
      int  disk_units;                 /* Number of disk units avail.   */
      int  tape_units;                 /* Number of tape units avail.   */
      int  cdrom_units;                /* Number of CDROM units avail.  */
      int  every_n_units;              /* If not zero, print #3 detail  */
                                       /*   output every n units        */
      char pro_algorithm[8];           /* FIFO : first-in, first-out    */
                                       /* IPRI : initial priority       */
                                       /* MLFQ : multi-level fb queue   */
    } ospr;

  struct input_data_record
    { int  process_priority;           /* User assigned priority        */
      int  process_memsize;            /* Load module memory requirement*/
      int  run_info[10][3];            /* 10 groups of 3 integers:      */
                                       /*    0 = CPU units              */
                                       /*    1 = I/O units              */
                                       /*    2 = I/O device types:      */
                                       /*        1 = DEV_DISK           */
                                       /*        2 = DEV_TAPE           */
                                       /*        3 = DEV_CD             */
                                       /*  0 thru 9 is the 10 cycles    */
      char process_name[8];            /* User name of process 7 chars  */
    } idr;



/***
 *** OS Data Structures
 ***/

  int pcbl_cur_size;
  int pcbl_cur_item;
  struct pcb_list
    { int  id;                         /* Unique process id             */
      char name[9] ;                   /* User name of process          */
      int  initial_priority;           /* User assigned priority        */
      int  memsize;                    /* Load module memory requirement*/
      int  run_info[10][3];            /* 10 groups of 3 integers:      */
                                       /*    0 = CPU units              */
                                       /*    1 = I/O units              */
                                       /*    2 = I/O device type        */
                                       /*        1 = DEV_DISK           */
                                       /*        2 = DEV_TAPE           */
                                       /*        3 = DEV_CD             */
                                       /*  0 thru 9 is the 10 cycles    */
      int  cur_ctr;                    /* Current CPU count for a cycle */
      char state[2];                   /* E=Exec,B=Blocked,R=Ready,     */
                                       /* D=Done, H=Held                */
      int  total_cpu;                  /* Total CPU currently used      */
      int  total_ios;                  /* Total I/Os currently used     */
      int  start_time;                 /* First Time Process Started    */
      int  end_time;                   /* Time Process Ended.           */
      int  sim_ri_idx;                 /* Current run_info index: 0-9   */
   } pcbl[MAX_ARRAY];

  int idl_cur_size;
  int idl_cur_item;
  struct input_data_list
    { char name[9];                    /* User name of process          */
      int  initial_priority;           /* User assigned priority        */
      int  memsize;                    /* Load module memory requirement*/
      int  run_info[10][3];            /* 10 groups of 3 integers:      */
                                       /*    0 = CPU units              */
                                       /*    1 = I/O units              */
                                       /*    2 = I/O device type        */
                                       /*        1 = DEV_DISK           */
                                       /*        2 = DEV_TAPE           */
                                       /*        3 = DEV_CD             */
                                       /*  0 thru 9 is the 10 cycles    */
    } idl[MAX_ARRAY];

  int eventl_cur_size;
  int eventl_cur_item;
  struct event_list
    { int  time;                       /* Time Event should be invoked  */
      int  pid;                        /* Process Id of event           */
      char type[2];                    /* Type of event:                */
                                       /*  T = Time Out                 */
                                       /*  I = I/O Completion           */
    } eventl[MAX_ARRAY];

  int dl_cur_size[DEV_ARRAY];
  int dl_cur_item[DEV_ARRAY];
  struct device_list
    { int  did;                        /* device ID. Unique within type */
      int  pid;                        /* current PID using this device */
      char state[2];                   /* State of device: Aval/Busy    */
    } dl[DEV_ARRAY][MAX_ARRAY];
      /* DEV_ARRAY's: 1 = DEV_DISK, 2 = DEV_TAPE, 3 = DEV_CD   */

  int rl_cur_size;
  int rl_cur_item;
  struct ready_list
    { int  pid;                        /* PID of process                */
      int  current_priority;           /* System assigned priority      */
    } rl[MAX_ARRAY];

  int bl_cur_size;
  int bl_cur_item;
  struct blocked_list
    { int  pid;                        /* PID of process                */
      int  current_priority;           /* System assigned priority      */
      char state[2];                   /* State of service request:     */
                                       /* IO_WORKING or IO_PENDING Dev  */
                                       /* or IO_DONE but not yet removed*/
      int  dtype;                      /* Device Type: DEV_DISK, etc.   */
      int  did;                        /* Device ID if IO_WORKING       */
    } bl[MAX_ARRAY];

  int fbml_cur_size;
  int fbml_cur_item;
  struct fixed_block_memory_list
    { int  state;                      /* MEM_FREE=0, MEM_USED=1        */
      int  pid;                        /* PID of process, or if OS's 0. */
    } fbml[MAX_MEM_ARRAY];

/***
 *** OS Variables
 ***/

  int  master_clock;                   /* Master system clock           */
  int  next_pid;                       /* Assigns next process id       */
  int  curr_pid;                       /* Contains pid of process exec. */
  char pgms_to_process[2];             /* Switch to stop OS simulation  */
  int  os_counter;                     /* Counts down for MASTER,DONES, */
                                       /*  and IOS for proper completion*/
  int  cpu_units_left;                 /* Number of units left since    */
                                       /*  process was last dispatched. */
  int  printed_input_data = 0;         /* Flag if printed input data    */
  int  num_os_parms = 0;               /* Number of OS Parms processed  */

/***
 *** Debugging variables
 ***/
  char debug_it[2];                     /* Is debugging turned on?       */


/***
 *** OS Routines I used
 ***/

/*********************************************************/
/*** allocate:                                         ***/
/*** Routine checks if resources exist to allocate     ***/
/***   another process and if so does it.              ***/
/*** Returns PID# (>0) if allocation done,             ***/
/***      else returns 0.                              ***/
/***                                                   ***/
/***  1) Uses FIFO or initial priority to              ***/
/***       schedule the idl.                           ***/
/***  2) Allocates memory to process.                  ***/
/***  3) Allocates pcbl to process.                    ***/
/***                                                   ***/
/*********************************************************/

/*********************************************************/
/*** check_args:
/*** Routine verifies that there are three arguments   ***/
/***  passed and they represent valid values.          ***/
/*** Returns 0 if everything OK, else returns -1.      ***/
/*** Parameters:                                       ***/
/***   1) O/S Parameter File Name                      ***/
/***   2) Input Queue File Name                        ***/
/***   3) "Y" or "N" for Debug Info                    ***/
/*** Optionally:                                       ***/
/***   /? which displays the usage of this program     ***/
/*********************************************************/

/*********************************************************/
/*** check_event:                                      ***/
/*** Routine checks if event set for this clock pulse  ***/
/***  If so trigger event, then remove event data.     ***/
/*** NOTE: Saves top event (index = 1) into index = 0  ***/
/***       array element to be used later!             ***/
/*********************************************************/

/*** dispatcher:
 *** Routine dispatches a ready process:
 ***  0) If a process has been HELD. It should be allocated first.
 ***       -> #1 Step occurs, except pcbl[0] holds the following:
 ***          id is the pid of the HELD process,
 ***          state is HELD,
 ***  1) If process ready, state = "R" then does:
 ***       -> Changes state to "E", removes process from ready list
 ***       -> Creates Event-Data record for process of type "T" time-out
 ***            The time calculation is master_clock+1+ospr.init_quantum.
 ***       -> Sets curr_pid to processes PID.
 ***  2) If process first time to execute, print "started at"
 ***  3) If no process in ready list and processes do exist, set
 ***       curr_pid to IDLE. (Must have an I/O Event pending)
 ***
 ***/

/*********************************************************/
/*** disp_help:                                        ***/
/*** Routine tells user they entered the   wrong types ***/
/***  of parameters, or wrong number, or they entered  ***/
/***  no parameters, or they entered /? parameter.     ***/
/*********************************************************/


/*** find_idl_to_allocate:
 *** Routine checks idl to find a process to load a pcbl.
 ***
 *** Returns index to idl item if so, else returns 0 if not.
 *** If mem_allocation_ok returns -1, not enough memory at this time
 *** If mem_allocation_ok returns -2, not enough memory ever
 *** If mem_allocation_ok returns  0, there is enough memory for process
 ***/

/*********************************************************/
/*** dev_type:                                         ***/
/*** Routine checks if it can allocate device          ***/
/***  required for passed device type and pid.         ***/
/*** Returns 0 if not, else returns device ID#.        ***/
/*********************************************************/

/*********************************************************/
/*** init_os:                                          ***/
/*** Routine establishes the operating system startup  ***/
/***  condition.  It re-opens the input_queue.         ***/
/*********************************************************/

/*********************************************************/
/*** io_check:                                         ***/
/*** Routine takes care of the I/O decrement &         ***/
/*** I/O totals.                                       ***/
/*********************************************************/

/*********************************************************/
/*** io_handler:                                       ***/
/*** Routine handles I/O completion interrupt.         ***/
/***                                                   ***/
/*********************************************************/

/*********************************************************/
/*** ios_routine:                                      ***/
/*** Routine actually performs I/O request for user.   ***/
/***                                                   ***/
/*********************************************************/

/***  master_scheduler:
 ***  Routine schedules processes from the input list to pcb list.
 ***  Does this for as many pcb spots as available.
 ***
 ***/

/*** mem_allocation_ok:
 *** Routine checks to be certain there is enough memory for proposed
 ***  process.
 ***
 *** Input parameter is idl index value.
 ***
 *** Returns 0 if there is, else -1 if not.
 *** If the system memory is not big enough to ever run process returns -2
 ***/

/*********************************************************/
/*** os_sim:                                           ***/
/*** Routine is the main OS similuation loop.          ***/
/*** It is invoked for each clock cycle.               ***/
/*********************************************************/

/*********************************************************/
/*** print_1:                                          ***/
/*** Routine prints the current O/S Parameter.         ***/
/***                                                   ***/
/*********************************************************/

/*********************************************************/
/*** print_3:                                          ***/
/*** Routine prints the detail every n units.          ***/
/***                                                   ***/
/*********************************************************/

/*********************************************************/
/*** remove_blocked:                                   ***/
/*** Routine removes the blocked list record for       ***/
/***  the passed index to that record.                 ***/
/*********************************************************/

/*********************************************************/
/*** remove_device:                                    ***/
/*** Routine makes the passed device type and id       ***/
/***  available for use again.                         ***/
/*********************************************************/

/*********************************************************/
/*** remove_event:                                     ***/
/*** Routine removes the event record for the passed   ***/
/***  process id.                                      ***/
/*********************************************************/

/*********************************************************/
/*** remove_process:                                   ***/
/*** Routine removes process from system.              ***/
/*** Then calls master_scheduler().                    ***/
/*********************************************************/

/*********************************************************/
/*** sc_handler:                                       ***/
/*** Routine handles user request for service.         ***/
/***                                                   ***/
/*********************************************************/

/*********************************************************/
/*** set_blockedlist:                                  ***/
/*** Routine establishes the blocked list in either    ***/
/***  FIFO, or IPRI order based on Os_Parm setting.    ***/
/*** Receives the pcb index of process to place on bl. ***/
/*********************************************************/

/*********************************************************/
/*** set_event:                                        ***/
/*** Routine sets event in the event list.             ***/
/***                                                   ***/
/*********************************************************/

/*********************************************************/
/*** set_readylist:                                    ***/
/*** Routine establishes the ready list in either      ***/
/***  FIFO, or IPRI order based on Os_Parm setting.    ***/
/*** Receives the pcb index of process to place on rl. ***/
/*********************************************************/

/*********************************************************/
/*** sort_idl:                                         ***/
/*** Routine sorts the input data list in initial      ***/
/***  priority order.                                  ***/
/*********************************************************/

/*** timer_handler:
 ***  Places timed-out process at end of ready list.
 ***
 ***/

/*** user_process:
 ***  Decrements CPU for this cycle.  At Zero triggers SC_request.
 ***
 ***/